pub mod direction;
pub mod selection;

use crate::{thread_count::ThreadCount, letterbox::Sample};
use direction::Direction;
use selection::*;

pub struct Controller {
    n: ThreadCount,
    changed: bool,
    step_size: f64,
    step_direction: Direction,
    max_threads: i32,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
    t_last: f64,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: ThreadCount::new(max_threads),
            changed: false,
            step_size: max_threads as f64,
            step_direction: Direction::Down,
            max_threads,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
            t_last: f64::MAX,
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let scores = samples.into_iter().map(|x| x.energy).collect();
        let tn = self.selection_algorithm.find_best(scores);

        if self.t_last < tn * 0.5 {
            // Fallen outside the corridor
            if self.step_size > 1.0 {
                self.step_direction = -self.step_direction;
                self.step_size *= 1.75;
            } else {
                self.move_towards_farthest_edge(0.75);
            }
        } else {
            if tn > self.t_last {
                // The previous iteration performed a bit better
                if self.changed {
                    // Only reverse direction if we actually changed n in the last iteration
                    self.step_direction = -self.step_direction;
                }
            }

            if self.step_size > 1.0 {
                self.step_size *= 0.5;
            } else {
                self.step_size = self.step_size.tanh();
                if self.step_size < 0.3 {
                    self.move_towards_farthest_edge(0.75)
                }
            }
        }

        self.changed = self.n.adjust(self.step_direction, self.step_size);
        self.t_last = if self.changed {
            tn
        } else {
            f64::min(self.t_last, tn)
        };

        self.n.round() as i32
    }

    fn move_towards_farthest_edge(&mut self, scale: f64) {
        let n_max = self.max_threads as f64;
        if *self.n <= n_max * 0.5 {
            self.step_direction = Direction::Up;
            self.step_size = (n_max - *self.n) * scale;
        } else {
            self.step_direction = Direction::Down;
            self.step_size = *self.n * scale;
        }
    }
}
