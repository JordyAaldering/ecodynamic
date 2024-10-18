use crate::controller::{Controller, Direction, FrequencyDist, SelectionAlgorithm, ThreadCount};
use crate::letterbox::Sample;

pub struct ControllerEnergy {
    n: ThreadCount,
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
            step_size: max_threads as f64,
            step_direction: Direction::Down,
            max_threads: max_threads as f64,
            selection_algorithm: Box::new(FrequencyDist::new(4, true)),
            t_last: 0.0,
        }
    }
}

impl Controller for ControllerEnergy {
    fn adjust_threads(&mut self, samples: Vec<Sample>) -> f64 {
        let scores = samples.into_iter().map(|x| x.energy).collect();
        let tn = self.selection_algorithm.find_best(scores);

        if tn > self.t_last * 1.50 {
            // Previous iteration performed a lot better
            self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
            self.step_size = self.max_threads * 0.5;
        } else {
            if tn > self.t_last * 1.02 {
                // Previous iteration performed (a bit) better
                self.step_direction = -self.step_direction;
            }

            if self.step_size > 0.16 {
                self.step_size = f64::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
                self.step_size = self.max_threads * 0.5;
            }
        }

        if tn < self.t_last || tn > self.t_last * 1.02 {
            // Only update after a significant change
            self.t_last = tn;
        }

        self.n += self.step_direction * self.step_size;
        *self.n
    }
}

#[inline]
fn towards_farthest_edge(n: f64, max_threads: f64) -> Direction {
    // Prefer to move up; typically we don't want to end up in a case where we are running single-threaded
    if n > max_threads * 0.65 {
        Direction::Down
    } else {
        Direction::Up
    }
}
