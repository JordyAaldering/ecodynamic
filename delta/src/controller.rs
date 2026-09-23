use ecodynamic_api::*;

use crate::*;

/// Delta-based, energy-optimising thread controller.
pub struct DeltaController {
    max_threads: u16,
    cur_threads: f32,
    step_size: f32,
    step_dir: Direction,
    t_prev: f32,
}

impl DeltaController {
    pub fn new(max_threads: u16) -> Self {
        Self {
            max_threads,
            cur_threads: max_threads as f32,
            step_size: 0.5,
            step_dir: Direction::Descending,
            t_prev: 0.0,
        }
    }
}

impl Controller for DeltaController {
    fn get_demand(&self, _index: usize) -> Demand {
        Demand {
            num_threads: self.num_threads(),
            powercap_pct: 1.0,
        }
    }

    fn evolve(&mut self, samples: Vec<Sample>) {
        let tn = self.score(samples);

        if tn > self.t_prev * 1.50 {
            self.reset();
        } else {
            if tn > self.t_prev {
                self.step_dir = !self.step_dir;
            }

            if self.step_size > 0.155 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.reset();
            }
        }

        self.t_prev = tn;
        self.cur_threads += self.step_dir * self.step_size;
        self.cur_threads = self.cur_threads.clamp(1.0, self.max_threads as f32);
    }
}

impl DeltaController {
    fn score(&self, samples: Vec<Sample>) -> f32 {
        let mut scores = samples.into_iter().map(|s| s.energy).collect();
        median(&mut scores)
    }

    fn num_threads(&self) -> u16 {
        (self.cur_threads.round() as u16).clamp(1, self.max_threads)
    }

    /// Reset step size and set direction towards the center.
    fn reset(&mut self) {
        self.step_size = 0.5 * self.max_threads as f32;
        self.step_dir = Direction::from(self.num_threads() < (self.max_threads / 2));
    }
}
