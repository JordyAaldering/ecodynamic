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
            selection_algorithm: Box::new(FrequencyDist::new(5, true)),
            t_last: f64::MAX,
        }
    }
}

impl Controller for ControllerEnergy {
    fn adjust_threads(&mut self, samples: Vec<Sample>) -> f64 {
        let scores = samples.into_iter().map(|x| x.energy).collect();
        let tn = self.selection_algorithm.find_best(scores);

        if tn > self.t_last * 1.5 {
            // The previous iteration performed a lot better
            // And the step size was not already reset in the previous iteration
            self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
            self.step_size = self.max_threads * 0.5;
        } else {
            if tn > self.t_last {
                // The previous iteration performed (a bit) better
                if self.changed {
                    // Only reverse direction if n changed in the previous iteration
                    self.step_direction = -self.step_direction;
                }
            }

            if self.step_size > 0.25001 {
                self.step_size = f64::max(self.step_size, f64::sqrt(self.step_size)) * 0.5;
            } else {
                self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
                self.step_size = self.max_threads * 0.5;
            }
        }

        self.t_last = if self.changed {
            tn
        } else {
            // Thread-count was not changed
            (self.t_last + tn) * 0.5
        };

        let prev_n = *self.n;
        self.n += self.step_direction * self.step_size;
        self.changed = prev_n.round() != self.n.round();

        *self.n
    }
}

#[inline]
fn towards_farthest_edge(n: f64, max_threads: f64) -> Direction {
    if n > max_threads * 0.5 {
        Direction::Down
    } else {
        Direction::Up
    }
}
