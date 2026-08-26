use clap::Parser;

use crate::*;

const MIN_STEPSIZE: f32 = 0.1;

pub struct CorridorController {
    samples: Vec<Sample>,
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
    #[arg(long, default_value = "frequency-dist")]
    pub filter: FilterFunction,
}

impl CorridorController {
    pub fn new(config: CorridorConfig, capabilities: &Capabilities) -> Self {
        Self {
            samples: Vec::with_capacity(config.letterbox_size),
            max_threads: capabilities.max_threads.max(1),
            cur_threads: capabilities.max_threads.max(1) as f32,
            step_size: capabilities.max_threads.max(1) as f32, // Will immediately be halved in the first iteration
            step_dir: Direction::Descending,
            t_prev: f32::MAX,
            t1: f32::MAX,
            config,
        }
    }
}

impl Controller for CorridorController {
    fn get_demand(&mut self) -> Demand {
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
        let scores = self.samples.iter().map(|s| s.runtime).collect();
        let tn = self.config.filter.select(scores);

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
        self.cur_threads = self.cur_threads.clamp(1.0, self.max_threads as f32);
    }

    /// Get the actual number of threads to use.
    fn num_threads(&self) -> u16 {
        (self.cur_threads.round() as u16).clamp(1, self.max_threads)
    }
}
