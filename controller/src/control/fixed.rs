use crate::message::{Demand, Sample};

use super::Controller;

pub struct FixedController {
    settings: FixedControllerSettings,
}

pub struct FixedControllerSettings {
    pub max_threads: i32,
}

impl FixedController {
    pub fn new(settings: FixedControllerSettings) -> Self {
        Self { settings }
    }
}

impl Controller for FixedController {
    fn sample_received(&mut self, _: Sample) { }

    fn next_demand(&mut self) -> Demand {
        Demand { num_threads: self.settings.max_threads }
    }
}
