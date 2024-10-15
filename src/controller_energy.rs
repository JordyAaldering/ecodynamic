use crate::controller::{Controller, Direction, FrequencyDist, SelectionAlgorithm, ThreadCount};
use crate::letterbox::Sample;

pub struct ControllerEnergy {
    n: ThreadCount,
    changed: bool,
    step_size: f64,
    step_direction: Direction,
    max_threads: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
    t_last: f64,
}

impl ControllerEnergy {
    pub fn new(max_threads: i32) -> ControllerEnergy {
        ControllerEnergy {
            n: ThreadCount::new(max_threads),
            changed: false,
            step_size: max_threads as f64,
            step_direction: Direction::Down,
            max_threads: max_threads as f64,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
            t_last: f64::MAX,
        }
    }
}

impl Controller for ControllerEnergy {
    fn adjust_threads(&mut self, samples: Vec<Sample>) -> f64 {
        let scores = samples.into_iter().map(|x| x.energy).collect();
        let tn = self.selection_algorithm.find_best(scores);

        if self.t_last < tn * 0.25 {
            self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
            self.step_size = self.max_threads * 0.5;
        } else {
            if tn > self.t_last {
                // The previous iteration performed a bit better
                //if self.changed {
                    // Only reverse direction if we actually changed n in the last iteration
                    self.step_direction = -self.step_direction;
                //}
            }

            if self.step_size > 1.0 {
                self.step_size *= 0.5;
            } else if self.step_size > 0.25 {
                self.step_size = self.step_size.tanh();
            } else {
                self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
                self.step_size = self.max_threads * 0.5;
            }
        }

        self.changed = self.n.adjust(self.step_direction, self.step_size);
        self.t_last = if self.changed {
            tn
        } else {
            f64::min(self.t_last, tn)
        };

        *self.n
    }
}

fn towards_farthest_edge(n: f64, max_threads: f64) -> Direction {
    if n > max_threads * 0.5 {
        Direction::Down
    } else {
        Direction::Up
    }
}
