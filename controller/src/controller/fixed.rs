use crate::{Demand, Sample};

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
    fn evolve(&mut self, _: Vec<Sample>) { }

    fn next_demand(&mut self) -> Demand {
        Demand { num_threads: self.num_threads, power_limit_uw: 0 }
    }
}
