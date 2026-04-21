use crate::{Capabilities, Controller, Demand, Sample};

pub struct FixedController {
    num_threads: u16,
}

impl FixedController {
    pub fn new(caps: &Capabilities) -> Self {
        Self {
            num_threads: caps.max_threads.unwrap_or(1),
        }
    }
}

impl Controller for FixedController {
    fn get_demand(&self) -> Demand {
        Demand {
            num_threads: self.num_threads,
            powercap_pct: 1.0,
        }
    }

    fn push_sample(&mut self, _: Sample) {}
}
