use crate::*;

pub struct OscilatingController {
    max_threads: u16,
    num_threads: u16,
    direction: Direction,
}

impl OscilatingController {
    pub fn new(capabilities: Capabilities) -> Self {
        let max_threads = capabilities.max_threads();
        Self {
            max_threads,
            num_threads: max_threads,
            direction: Direction::Descending,
        }
    }
}

impl Controller for OscilatingController {
    fn get_demand(&mut self) -> Demand {
        Demand {
            num_threads: self.num_threads,
            powercap_pct: 1.0,
        }
    }

    fn push(&mut self, _: Sample) {
        if self.direction == Direction::Ascending {
            self.num_threads = self.num_threads.saturating_add(1);
            if self.num_threads >= self.max_threads {
                self.num_threads = self.max_threads;
                self.direction = Direction::Descending;
            }
        } else {
            self.num_threads = self.num_threads.saturating_sub(1);
            if self.num_threads <= 1 {
                self.num_threads = 1;
                self.direction = Direction::Ascending;
            }
        }
    }
}
