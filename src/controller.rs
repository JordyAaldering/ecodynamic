mod direction;
mod selection;

use crate::letterbox::Sample;
use direction::Direction;
use selection::*;

use core::f64;

pub struct Controller {
    n: i32,
    t1: f64,
    t_last: f64,
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: max_threads,
            t1: f64::MAX,
            t_last: f64::MAX,
            step_size: max_threads,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let tn = self.selection_algorithm.find_best(samples) as f64;

        let speedup = self.t1 / tn;
        if speedup < (1.0 - self.corridor_width) {
            // We have fallen outside the corridor
            self.step_direction = Direction::Down;
            self.step_size = i32::max(1, self.n / 2);
        } else {
            if tn < self.t1 {
                // In the initial iteration t1 and t_last as f64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("Approximation of t1 updated to {}", tn);
                self.t1 = tn;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.n += self.step_direction * self.step_size;
        self.n = i32::max(1, i32::min(self.max_threads, self.n));
        self.t_last = tn;
        self.n
    }
}
