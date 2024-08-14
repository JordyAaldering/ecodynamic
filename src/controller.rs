mod direction;
mod selection;

use crate::letterbox::Sample;
use direction::Direction;
use selection::*;

pub struct Controller {
    n: i32,
    t1: u64,
    t_last: u64,
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f64,
    sample_value_selector: fn(Sample) -> u64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: max_threads,
            t1: u64::MAX,
            t_last: u64::MAX,
            step_size: max_threads,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            sample_value_selector: |sample| sample.energy_uj,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let tn = self.find_tn(samples);

        let speedup = self.t1 as f64 / tn as f64;
        if speedup < 1.0 - self.corridor_width {
            // We have fallen outside the corridor
            self.step_direction = Direction::Down;
            self.step_size = i32::max(1, self.n / 2);
        } else {
            if tn < self.t1 {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("Approximation of t1 updated to {}", tn);
                self.t1 = tn;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.t_last = tn;
        self.n = self.step_n();
        self.n
    }

    fn find_tn(&self, samples: Vec<Sample>) -> u64 {
        let samples = samples.into_iter().map(self.sample_value_selector).collect();
        self.selection_algorithm.find_best(samples)
    }

    fn step_n(&self) -> i32 {
        let n = self.n + self.step_direction * self.step_size;
        i32::max(1, i32::min(self.max_threads, n))
    }
}
