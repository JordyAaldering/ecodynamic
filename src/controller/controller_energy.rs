use crate::sample::Sample;

use super::{direction::Direction, Controller};

pub struct EnergyController {
    num_threads: f32,
    max_threads: f32,
    step_direction: Direction,
    step_size: f32,
    e_prev: f32,
}

impl EnergyController {
    pub fn new(max_threads: usize) -> Self {
        Self {
            num_threads: max_threads as f32,
            max_threads: max_threads as f32,
            step_direction: Direction::Down,
            step_size: max_threads as f32,
            e_prev: 0.0,
        }
    }
}

impl Controller for EnergyController {
    fn adjust_threads(&mut self, samples: Vec<Sample>) -> f32 {
        let e_avg = median(samples);

        if e_avg > self.e_prev * 1.50 {
            // Previous iteration performed a lot better
            self.reset_direction();
            self.reset_step_size();
        } else {
            if e_avg > self.e_prev {
                // Previous iteration performed (a bit) better
                self.step_direction = -self.step_direction;
            }

            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.reset_direction();
                self.reset_step_size();
            }
        }

        self.e_prev = e_avg;
        self.num_threads += self.step_direction * self.step_size;
        self.num_threads = self.num_threads.max(1.0).min(self.max_threads);
        self.num_threads
    }
}

impl EnergyController {
    /// Reset the step direction with a slight preference for increasing the thread count;
    /// since typically we don't want to end up in a case where we are single-threaded.
    #[inline]
    fn reset_direction(&mut self) {
        self.step_direction = if self.num_threads < self.max_threads * 0.65 {
            Direction::Up
        } else {
            Direction::Down
        };
    }

    /// Reset step size to half the number of maximum threads.
    #[inline]
    fn reset_step_size(&mut self) {
        self.step_size = self.max_threads * 0.5;
    }
}

fn median(mut samples: Vec<Sample>) -> f32 {
    let idx = samples.len() / 2;
    samples.sort_by(|a, b| a.energy.partial_cmp(&b.energy).unwrap());
    samples[idx].energy
}
