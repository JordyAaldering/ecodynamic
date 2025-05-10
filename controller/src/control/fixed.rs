use crate::message::Demand;

use super::Controller;

pub struct FixedController {
    max_threads: i32,
}

impl FixedController {
    pub fn new(max_threads: i32) -> Self {
        Self { max_threads }
    }
}

impl Controller for FixedController {
    fn evolve(&mut self, _scores: Vec<f32>) { }

    fn next_demand(&mut self) -> Demand {
        Demand { num_threads: self.max_threads }
    }
}
