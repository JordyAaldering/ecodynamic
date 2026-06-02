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
    stream: UnixStream,
    reader: BufReader<UnixStream>,
    region_uid: i32,
    measure_start: Option<(Instant, Rapl)>,
    after_fn: Option<Box<dyn Fn(Sample, &Demand)>>,
}

impl<I: Iterator> EcoIterator<I> {
    /// Create a new `EcoIterator` wrapping the given iterator and connecting to the controller with the given capabilities.
    /// It is allowed that no controller is running, in which case we simply ignore all controller interactions.
    pub fn new(inner: I, capabilities: Capabilities) -> io::Result<Self> {
        let mut stream = UnixStream::connect("/tmp/mtd_letterbox")?;
        let reader = BufReader::new(stream.try_clone()?);
        write_json_line(&mut stream, &capabilities)?;
        Ok(Self {
            inner,
            stream,
            reader,
            region_uid: REGION_COUNTER.fetch_add(1, Ordering::Relaxed),
            measure_start: None,
            after_fn: None,
        })
    }

    pub fn after_each_iteration<F>(mut self, f: F) -> Self
    where
        F: Fn(Sample, &Demand) + 'static,
    {
        self.after_fn = Some(Box::new(f));
        self
    }

    /// Send a signal to the controller that we are at the start of a parallel region.
    fn signal_start(&mut self) -> io::Result<Demand> {
        write_json_line(&mut self.stream, &Request {
            region_uid: self.region_uid,
            problem_size: None,
        })?;
        read_json_line(&mut self.reader)
    }

    /// Signal the end of the region and send runtime and energy results.
    fn signal_end(&mut self, sample: &Sample) -> io::Result<()> {
        write_json_line(&mut self.stream, sample)
    }
}

impl<I> Iterator for EcoIterator<I>
where
    I: Iterator,
{
    type Item = (Demand, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();

        if let Some(item) = item {
            let demand = if let Some(measure_start) = self.measure_start.take() {
                let sample = stop_measurements(self.region_uid, measure_start);
                self.signal_end(&sample).unwrap();
                let demand = self.signal_start().unwrap();

                if let Some(after_fn) = &self.after_fn {
                    after_fn(sample, &demand);
                }

                demand
            } else {
                // First iteration, only signal the start of the region, no sample to send yet
                self.signal_start().unwrap()
            };

            self.measure_start = Some(start_measurements());
            Some((demand, item))
        } else {
            // Last element; close the connection
            self.stream.shutdown(std::net::Shutdown::Both).unwrap();
            None
        }
    }
}

fn start_measurements() -> (Instant, Rapl) {
    let rapl = Rapl::new(false).unwrap();
    let now = Instant::now();
    (now, rapl)
}

fn stop_measurements(region_uid: i32, (time, rapl): (Instant, Rapl)) -> Sample {
    let runtime = time.elapsed();
    let energy = rapl.elapsed();
    Sample {
        region_uid,
        runtime: runtime.as_secs_f32(),
        energy: energy.values().sum(),
        usertime: None,
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
