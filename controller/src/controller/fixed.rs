use crate::{GlobalDemand, LocalDemand, Sample};

use super::Controller;

pub struct FixedController {
    threads_pct: f32,
}

impl FixedController {
    pub fn new() -> Self {
        Self { threads_pct: 1.0 }
    }
}

impl Controller for FixedController {
    fn evolve(&mut self, _: Vec<Sample>) { }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        let global = GlobalDemand::default();
        let local = LocalDemand { threads_pct: self.threads_pct };
        (global, local)
    }
}
