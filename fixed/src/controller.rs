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

impl Controller for FixedController {
    fn request(&self, _index: usize) -> Demand {
        Demand {
            num_threads: self.num_threads,
            powercap_pct: 1.0,
        }
    }

    fn evolve(&mut self, _: Vec<Sample>) {}
}
