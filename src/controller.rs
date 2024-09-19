mod direction;
mod selection;

use crate::{clamp::Clamp, letterbox::Sample};
use direction::Direction;
pub use selection::*;

pub struct Controller {
    n: Clamp<i32>,
    step_size: f64,
    step_direction: Direction,
    max_threads: i32,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
    t_last: f64,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: Clamp::new(max_threads, 1, max_threads),
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
                self.step_size = (self.n.value() / 2) as f64;
            }
        } else {
            if tn > self.t_last {
                // The previous iteration performed a bit better
                self.step_direction = -self.step_direction;
            }

            if self.step_size > 1.0 {
                self.step_size /= 2.0;
            } else {
                self.step_size = self.step_size.tanh();
                if self.step_size < 0.3 {
                    self.step_direction = Direction::towards(self.n.value(), self.max_threads / 2);
                    self.step_size = (self.max_threads / 2) as f64;
                }
            }
        }

        self.t_last = tn;
        self.n += f64::ceil(self.step_direction * self.step_size) as i32;
        self.n.value()
    }
}
