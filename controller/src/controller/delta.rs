use clap::Parser;

use crate::{Capabilities, Controller, Demand, Sample, direction::Direction, filter_functions::FilterFunction, score};

pub struct DeltaController {
    samples: Vec<Sample>,
    min_threads: u16,
    max_threads: u16,
    cur_threads: f32,
    step_size: f32,
    step_dir: Direction,
    t_prev: f32,
    config: DeltaConfig,
}

#[derive(Clone, Debug, Parser)]
pub struct DeltaConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,

    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while a value of 0 means that only runtime performance is optimised for.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.9)]
    pub energy_preference: f32,


    #[arg(long, default_value = "median")]
    pub select: FilterFunction,
}

impl DeltaController {
    pub fn new(config: DeltaConfig, capabilities: &Capabilities) -> Self {
        Self {
            samples: Vec::with_capacity(config.letterbox_size),
            min_threads: capabilities.min_threads,
            max_threads: capabilities.max_threads,
            cur_threads: capabilities.max_threads as f32,
            step_size: 0.5,
            step_dir: Direction::Descending,
            t_prev: 0.0,
            config,
        }
    }
}

impl Controller for DeltaController {
    fn get_demand(&self) -> Demand {
        Demand {
            num_threads: self.num_threads(),
            powercap_pct: 1.0,
        }
    }

    fn push_sample(&mut self, sample: Sample) {
        self.samples.push(sample);

        if self.samples.len() >= self.config.letterbox_size {
            self.evolve();
            self.samples.clear();
        }
    }
}

impl DeltaController {
    fn evolve(&mut self) {
        let tn = self.config.select.select(score(&self.samples, self.config.energy_preference));

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
        self.cur_threads = self.cur_threads.clamp(self.min_threads as f32, self.max_threads as f32);
    }

    /// Reset step size, and set direction towards the center.
    fn reset(&mut self) {
        self.step_size = 0.5 * self.max_threads as f32;
        self.step_dir = Direction::from(self.num_threads() < (self.max_threads / 2));
    }

    fn num_threads(&self) -> u16 {
        (self.cur_threads.round() as u16).clamp(self.min_threads, self.max_threads)
    }
}
