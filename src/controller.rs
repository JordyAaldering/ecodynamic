mod direction;
mod selection;

use std::{collections::HashMap, u64};

use crate::letterbox::Sample;
use direction::Direction;
use selection::*;

pub struct Controller {
    n: i32,
    t_best: u64,
    t_best_thread_count: u64,
    t_best_per_thread_count: HashMap<u64, u64>,
    t_last: u64,
    // TODO: step size (and n) as a float, so that we have less variation over time
    // e.g., now we can have [7,8,9,8,7,8,9,8,7,8,9] for a very long time
    // it would be nice if the longer it stays like that, the less it changes
    // of course we still want it to change sometimes to check for improvements
    // Potentially, after the value becomes very small make a large jump,
    // in order to escape local minima
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

fn energy_score(sample: Sample) -> u64 {
    return sample.energy_uj;

    #[allow(unreachable_code)]
    if sample.usertime_ns >= sample.realtime_ns {
        sample.energy_uj
    } else {
        let frac = sample.usertime_ns as f64 / sample.realtime_ns as f64;
        (sample.energy_uj as f64 * frac) as u64
    }
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: max_threads,
            t_best: u64::MAX,
            t_best_thread_count: max_threads as u64,
            t_best_per_thread_count: HashMap::with_capacity(max_threads as usize),
            t_last: u64::MAX,
            step_size: max_threads,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let samples = samples.into_iter().map(energy_score).collect();
        let tn = self.selection_algorithm.find_best(samples);

        // For analysing purposes
        if let Some(v) = self.t_best_per_thread_count.get_mut(&(self.n as u64)) {
            *v = u64::min(*v, tn);
        } else {
            self.t_best_per_thread_count.insert(self.n as u64, tn);
        }

        let speedup = self.t_best as f64 / tn as f64;
        if speedup < 1.0 - self.corridor_width {
            // Move up or down depending on where the best thread count was
            if tn > self.t_last {
                // The previous iteration performed better; reverse direction
                self.step_direction = -self.step_direction;
                //self.step_size = self.n / 2;
            } else {
                // Otherwise we move towards our estimated optimum
                self.step_direction = if self.n as u64 >= self.t_best_thread_count
                    { Direction::Down } else { Direction::Up };

                //self.step_size = i32::abs(self.n - self.t_best_thread_count as i32) - 1;
            }

            self.step_size = i32::max(1, self.n / 2);
            println!("Fallen outside the corridor (speedup = {}), step size to {}", speedup, self.step_size);
        } else {
            if speedup > 1.0 / (1.0 - self.corridor_width) {
                println!("Went above the corridor (speedup = {}), step size to {}", speedup, self.n / 2);
                self.step_size = self.n; // Will be n / 2 at the end of this block
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
        self.n
    }

    // The original runtime-based implementation, we use this for comparison
    #[allow(dead_code)]
    pub fn adjust_threads_runtime(&mut self, samples: Vec<Sample>) -> i32 {
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
        self.n
    }

    fn next_n(&self) -> i32 {
        let n = self.n + self.step_direction * self.step_size;
        i32::max(1, i32::min(self.max_threads, n))
    }
}

impl std::fmt::Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("t_best_per_thread_count: {:?}", self.t_best_per_thread_count))
    }
}
