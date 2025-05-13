use clap::Parser;

use crate::{Sample, ScoreFunction, SelectionFunction};

use super::Controller;

const UP: f32 = 1.0;
const DOWN: f32 = -1.0;

pub struct DeltaController {
    max_threads: f32,
    num_threads: f32,
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
    pub fn new(max_threads: f32, config: DeltaControllerConfig) -> Self {
        Self {
            max_threads,
            num_threads: max_threads,
            step_size: max_threads,
            step_dir: DOWN,
            e_prev: 0.0,
            config,
        }
    }

    /// Reset the step direction with a slight preference for increasing the thread count;
    /// since typically we don't want to end up in a case where we are single-threaded.
    fn reset_direction(&mut self) {
        self.step_dir = if self.num_threads < self.max_threads * 0.65 {
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
            self.step_size = self.max_threads * 0.5;
            self.reset_direction();
        } else {
            if e_next > self.e_prev {
                self.step_dir = -self.step_dir;
            }

            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_size = self.max_threads * 0.5;
                self.reset_direction();
            }
        }

        self.e_prev = e_next;
        self.num_threads += self.step_dir * self.step_size;
        self.num_threads = self.num_threads.max(1.0).min(self.max_threads);
    }

    fn num_threads(&mut self) -> i32 {
        self.num_threads.round() as i32
    }
}
