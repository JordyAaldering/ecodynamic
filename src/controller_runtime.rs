use crate::controller::{Controller, Direction, FrequencyDist, SelectionAlgorithm};
use crate::letterbox::Sample;

pub struct ControllerRuntime {
    n: i32,
    t1: f64,
    t_last: f64,
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

impl ControllerRuntime {
    pub fn new(max_threads: i32) -> ControllerRuntime {
        ControllerRuntime {
            n: max_threads,
            t1: f64::MAX,
            t_last: f64::MAX,
            step_size: max_threads,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5, false)),
        }
    }
}

impl Controller for ControllerRuntime {
    fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let samples = samples.into_iter().map(|x| x.runtime).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = self.t1 / tn;
        if speedup < (1.0 - self.corridor_width) * self.n as f64 {
            // We have fallen outside the corridor
            self.step_direction = Direction::Down;
            self.step_size = i32::max(1, self.n / 2);
        } else {
            if speedup > self.n as f64 {
                // In the initial iteration t1 and t_last are f64::MAX so we
                // reach this condition, an initialize t1 with an actual value
                self.t1 = tn * self.n as f64;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.t_last = tn;
        self.n += (self.step_direction * self.step_size as f64) as i32;
        self.n = i32::max(1, i32::min(self.max_threads, self.n));
        self.n
    }
}
