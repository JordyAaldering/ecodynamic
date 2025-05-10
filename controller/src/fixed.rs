use super::Controller;

pub struct FixedController {
    num_threads: i32,
}

impl FixedController {
    pub fn new(num_threads: i32) -> Self {
        Self { num_threads }
    }
}

impl Controller for FixedController {
    fn evolve(&mut self, _scores: Vec<f32>) { }

    fn num_threads(&mut self) -> i32 {
        self.num_threads
    }
}
