mod direction;
mod selection;

use direction::Direction;
use selection::{SelectionAlgorithm, FrequencyDist};

pub struct Controller {
    n: i32,
    t1: Option<u64>,
    t_last: u64,
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
            max_threads,
            n: max_threads,
            t1: None,
            t_last: 0,
            step_size: i32::max(max_threads / 2, 1),
            step_direction: Direction::Down,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist { num_ranges: 5 })
        }
    }

    pub fn adjust_threads(&mut self, runtime_results: Vec<u64>) -> i32 {
        let tn = self.selection_algorithm.find_best_time(runtime_results);

        if let Some(t1) = self.t1 {
            // Update
            self.n += self.step_direction as i32 * self.step_size;
            self.n = i32::clamp(self.n, 1, self.max_threads);

            let improvement = t1 as f64 / tn as f64;
            if improvement < self.n as f64 * self.corridor_width {
                self.step_direction = Direction::Down;
                self.step_size = i32::max(self.n / 2, 1);
            } else {
                if improvement > self.n as f64 {
                    self.t1 = Some(tn * self.n as u64);
                }

                if tn > self.t_last {
                    self.step_direction = -self.step_direction;
                }

                self.step_size = i32::max(self.step_size / 2, 1);
            }
        } else {
            // Init
            self.t1 = Some(tn * self.n as u64);
        }

        self.t_last = tn;
        self.n
    }
}
