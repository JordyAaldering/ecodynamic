mod sample;

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::atomic::{AtomicI32, Ordering},
};

pub use controller::{LocalDemand, Request, Sample};

use crate::sample::SamplePair;

static REGION_COUNTER: AtomicI32 = AtomicI32::new(0);

pub struct EcoIterator<I: Iterator> {
    inner: I,
    region_uid: i32,
    stream: Option<UnixStream>,
    sample_instant: Option<SamplePair>,
    before_fn: Option<fn(LocalDemand)>,
    after_fn: Option<fn(Sample)>,
}

impl<I: Iterator> EcoIterator<I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            region_uid: REGION_COUNTER.fetch_add(1, Ordering::Relaxed),
            stream: Some(UnixStream::connect("/tmp/mtd_letterbox").unwrap()),
            sample_instant: None,
            before_fn: None,
            after_fn: None,
        }
    }

    pub fn before(mut self, f: fn(LocalDemand)) -> Self {
        self.before_fn = Some(f);
        self
    }

    pub fn after(mut self, f: fn(Sample)) -> Self {
        self.after_fn = Some(f);
        self
    }

    /// Send a signal to the controller that we are at the start of a parallel region.
    fn signal_start(&mut self) -> SamplePair {
        if let Some(stream) = &mut self.stream {
            stream.write_all(&Request {
                region_uid: self.region_uid,
                problem_size: 0,
            }.to_bytes()).unwrap();

            let mut buf = [0u8; LocalDemand::SIZE];
            stream.read_exact(&mut buf).unwrap();
            if let Some(before_fn) = self.before_fn {
                let demand = LocalDemand::from(buf);
                before_fn(demand);
            }
        }

        SamplePair::start()
    }

    /// Signal the end of the region and send runtime and energy results.
    fn signal_end(&mut self, instant: SamplePair) {
        let sample = instant.stop(self.region_uid);
        if let Some(stream) = &mut self.stream {
            stream.write_all(&sample.to_bytes()).unwrap();
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
            if let Some(instant) = self.sample_instant.take() {
                // Send results of the previous region
                self.signal_end(instant);
            } else {
                // First element; do nothing
            }

            self.sample_instant = Some(self.signal_start());
        } else if let Some(stream) = &mut self.stream {
            // Last element; close connection
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }

        item
    }
}
