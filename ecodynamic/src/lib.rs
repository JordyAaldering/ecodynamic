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
    measure_start: Option<(Instant, Rapl)>,
    connection: Option<(UnixStream, BufReader<UnixStream>)>,
    after_fn: Option<Box<dyn Fn(Sample)>>,
}

impl<I: Iterator> EcoIterator<I> {
    /// Create a new `EcoIterator` wrapping the given iterator and connecting to the controller with the given capabilities.
    /// It is allowed that no controller is running, in which case we simply ignore all controller interactions.
    pub fn new(inner: I, capabilities: Capabilities) -> Self {
        let connection = if let Ok(mut stream) = UnixStream::connect("/tmp/mtd_letterbox") {
            write_json_line(&mut stream, &capabilities).unwrap();
            let reader = BufReader::new(stream.try_clone().unwrap());
            Some((stream, reader))
        } else {
            println!("Warning: could not connect to controller; running without controller");
            None
        };

        Self {
            inner,
            region_uid: REGION_COUNTER.fetch_add(1, Ordering::Relaxed),
            measure_start: None,
            connection,
            after_fn: None,
        }
    }

    pub fn for_each_sample<F>(mut self, f: F) -> Self
    where
        F: Fn(Sample) + 'static,
    {
        self.after_fn = Some(Box::new(f));
        self
    }

    /// Send a signal to the controller that we are at the start of a parallel region.
    fn signal_start(&mut self) -> Option<Demand> {
        if let Some((stream, reader)) = &mut self.connection {
            write_json_line(stream, &Request {
                region_uid: self.region_uid,
                problem_size: None,
            }).unwrap();
            Some(read_json_line(reader).unwrap())
        } else {
            None
        }
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

        if let Some((stream, _)) = &mut self.connection {
            write_json_line(stream, &sample).unwrap();
        }

        if let Some(for_each_sample_fn) = &self.after_fn {
            for_each_sample_fn(sample);
        }
    }
}

impl<I> Iterator for EcoIterator<I>
where
    I: Iterator,
{
    type Item = (Option<Demand>, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();

        if let Some(item) = item {
            if let Some((time, rapl)) = self.measure_start.take() {
                // Send results of the previous region
                self.signal_end(time, rapl);
            } else {
                // First element; do nothing
            }

            let demand = self.signal_start();
            self.measure_start = Some(start_measurements());
            Some((demand, item))
        } else {
            // Last element; close connection if a controller is connected
            if let Some((stream, _)) = &mut self.connection {
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
            None
        }
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

fn start_measurements() -> (Instant, Rapl) {
    let rapl = Rapl::new(false).unwrap();
    let now = Instant::now();
    (now, rapl)
}
