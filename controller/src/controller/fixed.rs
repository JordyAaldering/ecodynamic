use crate::*;

pub struct FixedController {
    num_threads: u16,
}

impl FixedController {
    pub fn new(capabilities: Capabilities) -> Self {
        Self {
            num_threads: capabilities.max_threads(),
        }
    }
}

impl Controller for FixedController {
    fn get_demand(&mut self) -> Demand {
        Demand {
            num_threads: self.num_threads,
            powercap_pct: 1.0,
        }
    }

    fn push(&mut self, _: Sample) {}
}
