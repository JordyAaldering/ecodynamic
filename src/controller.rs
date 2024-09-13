mod direction;
mod selection;

use crate::{clamp::Clamp, letterbox::Sample};
use direction::Direction;
use selection::*;

pub struct Controller {
    n: Clamp<i32>,
    step_size: Clamp<i32>,
    step_direction: Direction,
    max_threads: i32,
    corridor_width: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
    t_best: (f64, i32),
    t_last: f64,
}

fn user_frac(sample: &Sample) -> f64 {
    sample.usertime / (sample.runtime * sample.num_threads as f64)
}

fn energy_score(x: &Sample) -> f64 {
    user_frac(x) * x.energy
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: Clamp::new(max_threads, 1, max_threads),
            step_size: Clamp::new(max_threads, 1, max_threads),
            step_direction: Direction::Down,
            max_threads,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
            t_best: (f64::MAX, max_threads / 2),
            t_last: f64::MAX,
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let (t_best, t_best_thread_count) = self.t_best;

        let samples = samples.iter().map(energy_score).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = t_best / tn;

        if speedup < 1.0 - self.corridor_width {
            // Fallen outside the corridor: move towards found optimum
            self.step_direction = Direction::towards(self.n.get(), t_best_thread_count);
            self.step_size.set(self.max_threads / 2);

            //println!("Fallen outside the corridor (speedup = {}), step size to {}", speedup, self.step_size);
        } else {
            if tn < t_best {
                // In the initial ite as f64ration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                //println!("T_best updated to {} at {} threads", tn, self.n);
                self.t_best = (tn, self.n.get());
            }

            if tn > self.t_last {
                // The previous iteration performed better; reverse direction
                self.step_direction = -self.step_direction;
            }

            self.step_size /= 2;
        }

        self.t_last = tn;
        self.n += self.step_direction * self.step_size.get();
        self.n.get()
    }
}
