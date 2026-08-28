use clap::Parser;

use crate::*;

pub struct DeltaController {
    letterbox: Letterbox,
    filter: FilterFunction,
    max_threads: u16,
    cur_threads: f32,
    step_size: f32,
    step_dir: Direction,
    t_prev: f32,
}

#[derive(Clone, Debug, Parser)]
pub struct DeltaConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,
    #[arg(long, default_value = "median")]
    pub filter: FilterFunction,
}

impl DeltaController {
    pub fn new(config: DeltaConfig, capabilities: &Capabilities) -> Self {
        let max_threads = capabilities.max_threads.max(1);
        Self {
            letterbox: Letterbox::new(config.letterbox_size),
            filter: config.filter,
            max_threads,
            cur_threads: max_threads as f32,
            step_size: 0.5,
            step_dir: Direction::Descending,
            t_prev: 0.0,
        }
    }
}

impl Controller for DeltaController {
    fn get_demand(&mut self) -> Demand {
        Demand {
            num_threads: self.num_threads(),
            powercap_pct: 1.0,
        }
    }

    fn push(&mut self, sample: Sample) {
        if let Some(samples) = self.letterbox.push(sample) {
            let score = self.score(samples);
            self.evolve(score);
        }
    }
}

impl DeltaController {
    fn score(&self, samples: Vec<Sample>) -> f32 {
        let scores = samples.into_iter().map(|s| s.energy).collect();
        self.filter.select(scores)
    }

    fn evolve(&mut self, tn: f32) {
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

    /// Reset step size, and set direction towards the center.
    fn reset(&mut self) {
        self.step_size = 0.5 * self.max_threads as f32;
        self.step_dir = Direction::from(self.num_threads() < (self.max_threads / 2));
    }

    fn num_threads(&self) -> u16 {
        (self.cur_threads.round() as u16).clamp(1, self.max_threads)
    }
}
