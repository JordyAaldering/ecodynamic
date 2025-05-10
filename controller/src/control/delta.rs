use std::mem;

use crate::message::Demand;

use super::Controller;

const UP: f32 = 1.0;
const DOWN: f32 = -1.0;

pub struct DeltaController {
    samples: Vec<f32>,
    num_threads: f32,
    step_size: f32,
    step_dir: f32,
    e_prev: f32,
    settings: DeltaControllerSettings,
}

pub struct DeltaControllerSettings {
    pub max_threads: i32,
    pub population_size: usize,
}

impl DeltaController {
    pub fn new(settings: DeltaControllerSettings) -> Self {
        Self {
            samples: Vec::with_capacity(settings.population_size),
            num_threads: settings.max_threads as f32,
            step_size: settings.max_threads as f32,
            step_dir: DOWN,
            e_prev: 0.0,
            settings,
        }
    }

    /// Reset the step direction with a slight preference for increasing the thread count;
    /// since typically we don't want to end up in a case where we are single-threaded.
    fn reset_direction(&mut self) {
        self.step_dir = if self.num_threads < self.settings.max_threads as f32 * 0.65 {
            UP
        } else {
            DOWN
        };
    }
}

impl Controller for DeltaController {
    fn sample_received(&mut self, score: f32) {
        self.samples.push(score);
        if self.samples.len() >= self.settings.population_size {
            let mut samples_new = Vec::with_capacity(self.settings.population_size);
            mem::swap(&mut self.samples, &mut samples_new);
            self.evolve(samples_new);
        }
    }

    fn evolve(&mut self, scores: Vec<f32>) {
        let e_next = median(scores);

        if e_next > self.e_prev * 1.50 {
            self.step_size = self.settings.max_threads as f32 * 0.5;
            self.reset_direction();
        } else {
            if e_next > self.e_prev {
                self.step_dir = -self.step_dir;
            }

            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_size = self.settings.max_threads as f32 * 0.5;
                self.reset_direction();
            }
        }

        self.e_prev = e_next;
        self.num_threads += self.step_dir * self.step_size;
        self.num_threads = self.num_threads.max(1.0).min(self.settings.max_threads as f32);
    }

    fn get_demand(&self) -> Demand {
        Demand { num_threads: self.num_threads.round() as i32 }
    }
}

fn median(mut xs: Vec<f32>) -> f32 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs[xs.len() / 2]
}
