use crate::{Capabilities, Controller, Demand, Sample};

pub struct FixedController {
    num_threads: u16,
}

impl FixedController {
    pub fn new(capabilities: &Capabilities) -> Self {
        Self {
            num_threads: capabilities.max_threads,
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
