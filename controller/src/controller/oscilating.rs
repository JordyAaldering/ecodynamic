use crate::{GlobalDemand, LocalDemand, Sample};

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
    fn evolve(&mut self, _: Vec<Sample>) {
        self.num_threads += self.direction;
        if self.num_threads <= 1 || self.num_threads >= self.max_threads {
            self.direction = -self.direction;
        }
    }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        let global = GlobalDemand::default();
        let local = LocalDemand { num_threads: self.num_threads };
        (global, local)
    }
}
