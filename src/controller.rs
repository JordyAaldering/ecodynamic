mod direction;
mod selection;

use std::u64;

use crate::letterbox::Sample;
use direction::Direction;
use selection::*;

pub struct Controller {
    n: i32,
    t_best: u64,
    t_best_thread_count: u64,
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
            t_best: u64::MAX,
            t_best_thread_count: u64::MAX,
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

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> (i32, f64) {
        let samples = samples.into_iter().map(self.sample_value_selector).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = self.t_best as f64 / tn as f64;
        if speedup < 1.0 - self.corridor_width {
            println!("Fallen outside the corridor (speedup = {})", speedup);
            //self.step_direction = Direction::Down;
            // Move up or down depending on where the best thread count was
            self.step_direction = if self.n as u64 >= self.t_best_thread_count
                { Direction::Down } else { Direction::Up };
            self.step_size = i32::max(1, self.n / 2);
        } else {
            if speedup > 1.0 / (1.0 - self.corridor_width) {
                println!("Went above the corridor (speedup = {})", speedup);
                self.step_size = i32::max(1, self.n / 2);
            }

            if tn < self.t_best {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("T_best updated to {} at {} threads", tn, self.n);
                self.t_best = tn;
                self.t_best_thread_count = self.n as u64;
            }

            if tn > self.t_last {
                // The previous iteration performed better; reverse direction
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.n = self.next_n();
        self.t_last = tn;
        (self.n, speedup)
    }

    #[allow(dead_code)]
    pub fn adjust_threads_runtime(&mut self, samples: Vec<Sample>) -> (i32, f64) {
        let samples = samples.into_iter().map(|sample| sample.realtime_ns).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = self.t_best as f64 / tn as f64;
        if speedup < (1.0 - self.corridor_width) * self.n as f64 {
            // We have fallen outside the corridor
            self.step_direction = Direction::Down;
            self.step_size = i32::max(1, self.n / 2);
        } else {
            if speedup > self.n as f64 {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("Approximation of t1 updated to {}", tn * self.n as u64);
                self.t_best = tn * self.n as u64;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.n = self.next_n();
        self.t_last = tn;
        (self.n, speedup)
    }

    fn next_n(&self) -> i32 {
        let n = self.n + self.step_direction * self.step_size;
        i32::max(1, i32::min(self.max_threads, n))
    }
}

impl std::fmt::Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("T_best: {}, {} threads", self.t_best, self.t_best_thread_count))
    }
}
