use std::{
    io::{self, BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    sync::atomic::{AtomicI32, Ordering},
    time::Instant,
};

pub use controller::*;
use rapl_energy::Rapl;

static REGION_COUNTER: AtomicI32 = AtomicI32::new(0);

pub struct EcoIterator<I: Iterator> {
    inner: I,
    region_uid: i32,
    stream: Option<UnixStream>,
    reader: Option<BufReader<UnixStream>>,
    measure_start: Option<(Instant, Rapl)>,
    before_fn: Option<fn(Demand)>,
    after_fn: Option<fn(Sample)>,
}

impl<I: Iterator> EcoIterator<I> {
    pub fn new(inner: I, capabilities: Capabilities) -> Self {
        let mut stream = UnixStream::connect("/tmp/mtd_letterbox").unwrap();
        write_json_line(&mut stream, &capabilities).unwrap();
        let reader = BufReader::new(stream.try_clone().unwrap());

        Self {
            inner,
            region_uid: REGION_COUNTER.fetch_add(1, Ordering::Relaxed),
            stream: Some(stream),
            reader: Some(reader),
            measure_start: None,
            before_fn: None,
            after_fn: None,
        }
    }

    pub fn before(mut self, f: fn(Demand)) -> Self {
        self.before_fn = Some(f);
        self
    }

    pub fn after(mut self, f: fn(Sample)) -> Self {
        self.after_fn = Some(f);
        self
    }

    /// Send a signal to the controller that we are at the start of a parallel region.
    fn signal_start(&mut self) -> (Instant, Rapl) {
        if let (Some(stream), Some(reader)) = (&mut self.stream, &mut self.reader) {
            write_json_line(stream, &Request {
                region_uid: self.region_uid,
                problem_size: None,
            }).unwrap();

            if let Some(before_fn) = self.before_fn {
                before_fn(read_json_line(reader).unwrap());
            } else {
                let _: Demand = read_json_line(reader).unwrap();
            }
        }

        let rapl = Rapl::new(false).unwrap();
        let now = Instant::now();
        (now, rapl)
    }

    /// Signal the end of the region and send runtime and energy results.
    fn signal_end(&mut self, time: Instant, rapl: Rapl) {
        let runtime = time.elapsed();
        let energy = rapl.elapsed();
        let sample = Sample {
            region_uid: self.region_uid,
            runtime: runtime.as_secs_f32(),
            energy: energy.values().sum(),
            usertime: None,
        };

        if let Some(stream) = &mut self.stream {
            write_json_line(stream, &sample).unwrap();
        }

        if let Some(after_fn) = self.after_fn {
            after_fn(sample);
        }
    }
}

impl<I: Iterator> Iterator for EcoIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();

        if item.is_some() {
            if let Some((time, rapl)) = self.measure_start.take() {
                // Send results of the previous region
                self.signal_end(time, rapl);
            } else {
                // First element; do nothing
            }

            self.measure_start = Some(self.signal_start());
        } else if let Some(stream) = &mut self.stream {
            // Last element; close connection
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }

        item
    }
}

fn write_json_line<T: serde::Serialize>(stream: &mut UnixStream, message: &T) -> io::Result<()> {
    serde_json::to_writer(&mut *stream, message).map_err(io::Error::other)?;
    stream.write_all(b"\n")
}

fn read_json_line<T: serde::de::DeserializeOwned>(reader: &mut BufReader<UnixStream>) -> io::Result<T> {
    let mut line = String::new();
    let read = reader.read_line(&mut line)?;
    if read == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "socket closed while reading JSON message",
        ));
    }

    serde_json::from_str(line.trim_end()).map_err(io::Error::other)
}
