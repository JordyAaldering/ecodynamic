use crate::{controller::Controller, sample::Sample};

pub struct FixedController {
    num_threads: f32,
}

impl FixedController {
    pub fn new(num_threads: usize) -> Self {
        Self { num_threads: num_threads as f32 }
    }
}

impl Controller for FixedController {
    fn adjust_threads(&mut self, _: Vec<Sample>) -> f32 {
        self.num_threads
    }
}
