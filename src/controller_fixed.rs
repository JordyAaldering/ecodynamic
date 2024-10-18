use crate::controller::Controller;

pub struct FixedController {
    num_threads: usize,
}

impl FixedController {
    pub fn new(num_threads: usize) -> Self {
        Self { num_threads }
    }
}

impl Controller for FixedController {
    fn adjust_threads(&mut self, _: f32) -> f32 {
        self.num_threads as f32
    }
}
