use crate::message::Demand;

use super::Controller;

pub struct OscilatingController {
    max_threads: i32,
    num_threads: i32,
    direction: i32,
}

impl OscilatingController {
    pub fn new(max_threads: i32) -> Self {
        Self {
            max_threads,
            num_threads: max_threads,
            direction: -1,
        }
    }
}

impl Controller for OscilatingController {
    /// Interval depends on the number of samples; which is handled from the calling side
    fn evolve(&mut self, _scores: Vec<f32>) {
        self.num_threads += self.direction;

        // Swap direction if we are at an edge
        if self.num_threads <= 1 || self.num_threads >= self.max_threads {
            self.direction = -self.direction;
        }
    }

    fn next_demand(&mut self) -> Demand {
        Demand { num_threads: self.num_threads }
    }
}
