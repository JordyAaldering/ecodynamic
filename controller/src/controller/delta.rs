use clap::Parser;

use crate::{GlobalDemand, LocalDemand, Sample, ScoreFunction, SelectionFunction};

use super::Controller;

const UP: f32 = 1.0;
const DOWN: f32 = -1.0;

const THREADS_PCT_MIN: f32 = 0.1;

pub struct DeltaController {
    threads_pct: f32,
    step_size: f32,
    step_dir: f32,
    e_prev: f32,
    config: DeltaControllerConfig,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct DeltaControllerConfig {
    #[arg(long)]
    pub score: ScoreFunction,

    #[arg(long)]
    pub select: SelectionFunction,
}

impl DeltaController {
    pub fn new(config: DeltaControllerConfig) -> Self {
        Self {
            threads_pct: 1.0,
            step_size: 0.5,
            step_dir: DOWN,
            e_prev: 0.0,
            config,
        }
    }

    fn reset_direction(&mut self) {
        self.step_dir = if self.threads_pct < (1.0 + THREADS_PCT_MIN) / 2.0 {
            UP
        } else {
            DOWN
        };
    }
}

impl Controller for DeltaController {
    fn evolve(&mut self, samples: Vec<Sample>) {
        let e_next = self.config.select.select(self.config.score.score(samples));

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
        self.threads_pct += self.step_dir * self.step_size;
        self.threads_pct = self.threads_pct.max(THREADS_PCT_MIN).min(1.0);
    }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        let global = GlobalDemand::default();
        let local = LocalDemand { threads_pct: self.threads_pct };
        (global, local)
    }
}
