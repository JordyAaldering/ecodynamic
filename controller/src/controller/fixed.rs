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
    fn get_demand(&self) -> Demand {
        Demand::new()
            .with_threads(Some(self.num_threads))
    }

    fn push_sample(&mut self, _: Sample) {}
}
