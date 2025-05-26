use crate::{GlobalDemand, LocalDemand, Sample};

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

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        let global = GlobalDemand::default();
        let local = LocalDemand { num_threads: self.num_threads };
        (global, local)
    }
}
