use crate::{GlobalDemand, Demand, Sample};

use super::Controller;

const THREADS_PCT_MIN: f32 = 0.1;

pub struct OscilatingController {
    threads_pct: f32,
    direction: f32,
}

impl OscilatingController {
    pub fn new() -> Self {
        Self {
            threads_pct: 1.0,
            direction: -0.1,
        }
    }
}

impl Controller for OscilatingController {
    fn evolve(&mut self, _: Vec<Sample>) {
        self.threads_pct += self.direction;
        if self.threads_pct <= THREADS_PCT_MIN || self.threads_pct >= 1.0 {
            self.threads_pct = self.threads_pct.max(THREADS_PCT_MIN).min(1.0);
            self.direction = -self.direction;
        }
    }

    fn next_demand(&mut self) -> (GlobalDemand, Demand) {
        let global = GlobalDemand::default();
        let local = Demand { threads_pct: self.threads_pct };
        (global, local)
    }
}
