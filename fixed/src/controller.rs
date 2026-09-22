use crate::*;

/// Identity controller that always returns the same configuration.
pub struct FixedController {
    num_threads: u16,
}

impl FixedController {
    pub fn new(capabilities: &AppCapabilities) -> Self {
        Self {
            num_threads: capabilities.max_threads,
        }
    }
}

impl FixedController {
    pub fn get_demand(&self) -> Demand {
        Demand::new().with_threads(Some(self.num_threads))
    }

    pub fn push_sample(&mut self, _: Sample) {}
}
