use clap::Parser;

use crate::{Capabilities, Controller, Demand, filter_functions::FilterFunction, Sample, direction::Direction, score};

const MIN_STEPSIZE: f32 = 0.1;

pub struct CorridorController {
    samples: Vec<Sample>,
    min_threads: u16,
    max_threads: u16,
    cur_threads: f32,
    step_size: f32,
    step_dir: Direction,
    t_prev: f32,
    t1: f32,
    config: CorridorConfig,
}

#[derive(Clone, Debug, Parser)]
pub struct CorridorConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,

    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while a value of 0 means that only runtime performance is optimised for.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.9)]
    pub energy_preference: f32,

    #[arg(long, default_value = "frequency-dist")]
    pub select: FilterFunction,
}

impl CorridorController {
    pub fn new(config: CorridorConfig, capabilities: &Capabilities) -> Self {
        Self {
            samples: Vec::with_capacity(config.letterbox_size),
            min_threads: capabilities.min_threads,
            max_threads: capabilities.max_threads,
            cur_threads: capabilities.max_threads as f32,
            step_size: capabilities.max_threads as f32, // Will immediately be halved in the first iteration
            step_dir: Direction::Descending,
            t_prev: f32::MAX,
            t1: f32::MAX,
            config,
        }
    }
}

impl Controller for CorridorController {
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

impl CorridorController {
    fn evolve(&mut self) {
        let tn = self.config.select.select(score(&self.samples, self.config.energy_preference));

        let speedup = self.t1 / (tn + f32::EPSILON);
        if speedup < 0.5 * self.num_threads() as f32 {
            // We have fallen below the corridor; reset step size and direction
            self.step_size = (0.5 * self.cur_threads).max(MIN_STEPSIZE);
            self.step_dir = Direction::Descending;
        } else {
            if speedup > self.num_threads() as f32 {
                // In the initial iteration t1 and t_last are f64::MAX so we
                // reach this condition, an initialize t1 with an actual value
                self.t1 = tn * (self.num_threads() as f32);
            }

            if tn > self.t_prev {
                self.step_dir = !self.step_dir;
            }

            // Halve the step size
            self.step_size = (0.5 * self.step_size).max(MIN_STEPSIZE);
        }

        self.t_prev = tn;
        self.cur_threads += self.step_dir * self.step_size;
        self.cur_threads = self.cur_threads.clamp(self.min_threads as f32, self.max_threads as f32);
    }

    /// Get the actual number of threads to use.
    fn num_threads(&self) -> u16 {
        (self.cur_threads.round() as u16).clamp(self.min_threads, self.max_threads)
    }
}
