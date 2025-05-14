use clap::Parser;

use crate::{Sample, ScoreFunction, SelectionFunction};

use super::Controller;

pub struct CorridorController {
    max_threads: i32,
    num_threads: i32,
    step_size: i32,
    step_dir: i32,
    t_prev: f32,
    t1: f32,
    config: CorridorControllerConfig,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct CorridorControllerConfig {
    #[arg(long)]
    pub score: ScoreFunction,

    #[arg(long)]
    pub select: SelectionFunction,
}

impl CorridorController {
    pub fn new(max_threads: i32, config: CorridorControllerConfig) -> Self {
        Self {
            max_threads,
            num_threads: max_threads,
            step_size: max_threads,
            step_dir: -1,
            t_prev: f32::MAX,
            t1: f32::MAX,
            config,
        }
    }
}

impl Controller for CorridorController {
    fn evolve(&mut self, samples: Vec<Sample>) {
        let tn = self.config.select.select(self.config.score.score(samples));

        if self.t1 / tn < 0.5 * self.num_threads as f32 {
            self.step_size = i32::max(1, self.num_threads / 2);
            self.step_dir = -1;
        } else {
            if self.t1 / tn > self.num_threads as f32 {
                self.t1 = tn * self.num_threads as f32;
            }

            if tn > self.t_prev {
                self.step_dir = -self.step_dir;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.t_prev = tn;
        self.num_threads += self.step_dir * self.step_size;
        self.num_threads = self.num_threads.max(1).min(self.max_threads);
    }

    fn num_threads(&mut self) -> i32 {
        self.num_threads
    }
}
