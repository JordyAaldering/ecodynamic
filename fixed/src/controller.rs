use crate::*;

/// Identity controller that always returns the same configuration.
pub struct FixedController {
    num_threads: u16,
}

impl FixedController {
    pub fn new(num_threads: u16) -> Self {
        Self {
            num_threads,
        }
    }
}

impl FixedController {
    pub fn get_demand(&self) -> Demand {
        Demand {
            num_threads: self.num_threads,
            powercap_pct: 1.0,
        }
    }

    pub fn push(&mut self, _: Sample) {}
}
