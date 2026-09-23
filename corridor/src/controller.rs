use clap::Parser;
use ecodynamic_api::*;
use ecodynamic_core::*;

const MIN_STEPSIZE: f32 = 0.1;

/// Corridor-based, runtime-optimising thread controller.
pub struct CorridorController {
    letterbox: Letterbox,
    max_threads: u16,
    cur_threads: f32,
    step_size: f32,
    step_dir: Direction,
    t_prev: f32,
    t1: f32,
}

#[derive(Clone, Debug, Parser)]
pub struct CorridorConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,
}

impl CorridorController {
    pub fn new(config: &CorridorConfig, capabilities: &AppCapabilities) -> Self {
        let max_threads = capabilities.max_threads;
        Self {
            letterbox: Letterbox::new(config.letterbox_size),
            max_threads,
            cur_threads: max_threads as f32,
            step_size: max_threads as f32, // Will immediately be halved in the first iteration
            step_dir: Direction::Descending,
            t_prev: f32::MAX,
            t1: f32::MAX,
        }
    }
}

impl CorridorController {
    pub fn get_demand(&self) -> Demand {
        Demand {
            num_threads: self.num_threads(),
            powercap_pct: 1.0,
        }
    }

    pub fn push(&mut self, sample: Sample) {
        if let Some(samples) = self.letterbox.push(sample) {
            let score = self.score(samples);
            self.evolve(score);
        }
    }

    fn score(&self, samples: Vec<Sample>) -> f32 {
        let scores = samples.into_iter().map(|s| s.runtime).collect();
        frequency_dist(scores, 5)
    }

    fn evolve(&mut self, tn: f32) {
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

    fn num_threads(&self) -> u16 {
        (self.cur_threads.round() as u16).clamp(1, self.max_threads)
    }
}
