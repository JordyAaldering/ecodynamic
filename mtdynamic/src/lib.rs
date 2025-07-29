mod sample;

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::atomic::{AtomicI32, Ordering},
};

use controller::{Demand, Request, Sample};

use crate::sample::SampleInstant;

static REGION_COUNTER: AtomicI32 = AtomicI32::new(0);

pub struct MtdIterator<I: Iterator> {
    inner: I,
    region_uid: i32,
    stream: Option<UnixStream>,
    sample_instant: Option<SampleInstant>,
    before_fn: Option<fn(Demand)>,
    after_fn: Option<fn(Sample)>,
}

impl<I: Iterator> MtdIterator<I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            region_uid: REGION_COUNTER.fetch_add(1, Ordering::Relaxed),
            stream: UnixStream::connect("/tmp/mtd_letterbox").ok(),
            sample_instant: None,
            before_fn: None,
            after_fn: None,
        }
    }

    pub fn before(&mut self, f: fn(Demand)) {
        self.before_fn = Some(f);
    }

    pub fn after(&mut self, f: fn(Sample)) {
        self.after_fn = Some(f);
    }

    /// Send a signal to the controller that we are at the start of a parallel region.
    fn signal_start(&mut self) -> SampleInstant {
        if let Some(stream) = &mut self.stream {
            stream.write_all(&Request {
                region_uid: self.region_uid,
                problem_size: 0,
            }.to_bytes()).unwrap();

            let mut buf = [0u8; Demand::SIZE];
            stream.read_exact(&mut buf).unwrap();
            if let Some(before_fn) = self.before_fn {
                let demand = Demand::from(buf);
                before_fn(demand);
            }
        }

        SampleInstant::start()
    }

    /// Signal the end of the region and send runtime and energy results.
    fn signal_end(&mut self, instant: SampleInstant) {
        let sample = instant.stop(self.region_uid);
        if let Some(stream) = &mut self.stream {
            stream.write_all(&sample.to_bytes()).unwrap();
        }

        if let Some(after_fn) = self.after_fn {
            after_fn(sample);
        }
    }
}

impl<I: Iterator> Iterator for MtdIterator<I> {
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
