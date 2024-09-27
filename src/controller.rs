mod direction;
mod selection;

use crate::{clamp::Clamp, letterbox::Sample};
use direction::Direction;
use selection::*;

pub struct Controller {
    n: Clamp<f64>,
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
            n: Clamp::new(max_threads as f64, 1.0, max_threads as f64),
            changed: false,
            step_size: max_threads as f64,
            step_direction: Direction::Down,
            max_threads,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
            t_last: f64::MAX,
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let scores = samples.into_iter().map(|x| x.energy_score()).collect();
        let tn = self.selection_algorithm.find_best(scores);

        if self.t_last < tn * 0.5 {
            // Fallen outside the corridor
            if self.step_size > 1.0 {
                self.step_direction = -self.step_direction;
                self.step_size *= 1.75;
            } else {
                self.step_direction = Direction::Down;
                self.step_size = *self.n * 0.5;
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
                    self.step_direction = Direction::towards(*self.n as i32, self.max_threads / 2);
                    self.step_size = (self.max_threads / 2) as f64;
                }
            }
        }

        let prev_n = *self.n;
        self.n += self.step_direction * self.step_size;

        self.changed = prev_n.round() as i32 != (*self.n).round() as i32;
        if self.changed {
            self.t_last = tn;
        } else {
            self.t_last = f64::min(self.t_last, tn);
        }

        (*self.n).round() as i32
    }
}
