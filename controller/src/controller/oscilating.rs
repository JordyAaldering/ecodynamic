use crate::{Capabilities, Demand, Sample, direction::Direction};

use super::Controller;

pub struct OscilatingController {
    min_threads: u16,
    max_threads: u16,
    num_threads: u16,
    direction: Direction,
}

impl OscilatingController {
    pub fn new(capabilities: &Capabilities) -> Self {
        Self {
            min_threads: capabilities.min_threads,
            max_threads: capabilities.max_threads,
            num_threads: capabilities.max_threads,
            direction: Direction::Descending,
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

    fn push_sample(&mut self, _: Sample) {
        if self.direction == Direction::Ascending {
            self.num_threads = self.num_threads.saturating_add(1);
            if self.num_threads >= self.max_threads {
                self.num_threads = self.max_threads;
                self.direction = Direction::Descending;
            }
        } else {
            self.num_threads = self.num_threads.saturating_sub(1);
            if self.num_threads <= self.min_threads {
                self.num_threads = self.min_threads;
                self.direction = Direction::Ascending;
            }
        }
    }
}
