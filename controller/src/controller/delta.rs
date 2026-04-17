use clap::Parser;

use crate::{Capabilities, Controller, Demand, Direction, FilterFunction, Sample, get_scores};

const THREADS_PCT_MIN: f32 = 0.1;

pub struct DeltaController {
    samples: Vec<Sample>,
    threads_pct: f32,
    max_threads: u16,
    step_size: f32,
    step_dir: Direction,
    e_prev: f32,
    config: DeltaControllerConfig,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct DeltaControllerConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,

    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while a value of 0 means that only runtime performance is optimised for.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.9)]
    pub energy_preference: f32,

    #[arg(long)]
    pub select: FilterFunction,
}

impl DeltaController {
    pub fn new(config: DeltaControllerConfig, caps: &Capabilities) -> Self {
        Self {
            samples: Vec::with_capacity(config.letterbox_size),
            threads_pct: 1.0,
            max_threads: caps.max_threads.unwrap_or(1),
            step_size: 0.5,
            step_dir: Direction::Decreasing,
            e_prev: 0.0,
            config,
        }
    }
}

impl Controller for DeltaController {
    fn get_demand(&self) -> Demand {
        Demand {
            num_threads: ((self.threads_pct * self.max_threads as f32).round() as u16).max(1),
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
        let e_next = self.config.select.select(get_scores(&self.samples, self.config.energy_preference));

        if e_next > self.e_prev * 1.50 {
            self.step_size = 0.5;
            self.reset_direction();
        } else {
            if e_next > self.e_prev {
                self.step_dir = -self.step_dir;
            }

            // TODO: this needs to be updated for step_size in range (0,1] instead of range [1,max_threads]
            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_size = 0.5;
                self.reset_direction();
            }
        }

        self.e_prev = e_next;
        self.threads_pct += Into::<f32>::into(self.step_dir) * self.step_size;
        self.threads_pct = self.threads_pct.max(THREADS_PCT_MIN).min(1.0);
    }

    fn reset_direction(&mut self) {
        self.step_dir = if self.threads_pct < (1.0 + THREADS_PCT_MIN) / 2.0 {
            Direction::Increasing
        } else {
            Direction::Decreasing
        };
    }
}
