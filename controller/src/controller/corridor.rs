use clap::Parser;

use crate::{GlobalDemand, LocalDemand, Sample, ScoreFunction, SelectionFunction};

use super::Controller;

const _UP: f32 = 1.0;
const DOWN: f32 = -1.0;

const THREADS_PCT_MIN: f32 = 0.1;

pub struct CorridorController {
    threads_pct: f32,
    step_size: f32,
    step_dir: f32,
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
    pub fn new(config: CorridorControllerConfig) -> Self {
        Self {
            threads_pct: 1.0,
            step_size: 1.0, // Will immediately be halved in the first iteration
            step_dir: DOWN,
            t_prev: f32::MAX,
            t1: f32::MAX,
            config,
        }
    }
}

impl Controller for CorridorController {
    fn evolve(&mut self, samples: Vec<Sample>) {
        let tn = self.config.select.select(self.config.score.score(samples));

        // TODO: check if replacing num_threads with threads_pct here was sufficient, or if we need to update the formula
        if self.t1 / tn < 0.5 * self.threads_pct {
            self.step_size = f32::max(THREADS_PCT_MIN, self.threads_pct / 2.0);
            self.step_dir = DOWN;
        } else {
            if self.t1 / tn > self.threads_pct {
                self.t1 = tn * self.threads_pct;
            }

            if tn > self.t_prev {
                self.step_dir = -self.step_dir;
            }

            self.step_size = f32::max(THREADS_PCT_MIN, self.threads_pct / 2.0);
        }

        self.t_prev = tn;
        self.threads_pct += self.step_dir * self.step_size;
        self.threads_pct = self.threads_pct.max(THREADS_PCT_MIN).min(1.0);
    }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        let global = GlobalDemand::default();
        let local = LocalDemand { threads_pct: self.threads_pct };
        (global, local)
    }
}
