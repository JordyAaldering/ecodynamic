use crate::{Capabilities, Demand, Sample};

use super::Controller;

pub struct OscilatingController {
    min_threads: u16,
    max_threads: u16,
    num_threads: u16,
    ascending: bool,
}

impl OscilatingController {
    pub fn new(caps: &Capabilities) -> Self {
        Self {
            min_threads: caps.min_threads.unwrap_or(1),
            max_threads: caps.max_threads.unwrap_or(1),
            num_threads: caps.max_threads.unwrap_or(1),
            ascending: false,
        }
    }
}

impl Controller for OscilatingController {
    fn get_demand(&self) -> Demand {
        Demand {
            powercap_pct: 1.0,
            num_threads: self.num_threads,
        }
    }

    /// Ignores the sample, always evolve to the next step
    fn push_sample(&mut self, _: Sample) {
        if self.ascending {
            self.num_threads = self.num_threads.saturating_add(1);
            if self.num_threads >= self.max_threads {
                self.num_threads = self.max_threads;
                self.ascending = false;
            }
        } else {
            self.num_threads = self.num_threads.saturating_sub(1);
            if self.num_threads <= self.min_threads {
                self.num_threads = self.min_threads;
                self.ascending = true;
            }
        }
    }
}
