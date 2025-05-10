use crate::message::Demand;

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
    fn evolve(&mut self, _scores: Vec<f32>) { }

    fn get_demand(&self) -> Demand {
        Demand { num_threads: self.settings.max_threads }
    }
}
