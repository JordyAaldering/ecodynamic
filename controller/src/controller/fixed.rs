use crate::{Controller, GlobalDemand, LocalDemand, Sample};

pub struct FixedController {
    powercap_pct: f32,
    threads_pct: f32,
}

impl FixedController {
    pub fn new() -> Self {
        Self {
            powercap_pct: 1.0,
            threads_pct: 1.0,
        }
    }
}

impl Controller for FixedController {
    fn get_demand(&self) -> (GlobalDemand, LocalDemand) {
        let global = GlobalDemand { powercap_pct: self.powercap_pct };
        let local = LocalDemand { threads_pct: self.threads_pct };
        (global, local)
    }

    fn push_sample(&mut self, _: Sample) { }
}
