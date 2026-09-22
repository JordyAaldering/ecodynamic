use clap::Parser;
use ecocore::*;

use crate::*;

const MIN_STEPSIZE: f32 = 0.1;

/// Corridor-based thread controller.
pub struct Controller {
    letterbox: Letterbox,
    max_threads: u16,
    cur_threads: f32,
    step_size: f32,
    step_dir: Direction,
    t_prev: f32,
    t1: f32,
}

#[derive(Clone, Debug, Parser)]
pub struct Config {
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,
}

impl Controller {
    pub fn new(config: &Config, capabilities: &AppCapabilities) -> Self {
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

impl Controller {
    pub fn get_demand(&self) -> Demand {
        Demand::new()
            .with_threads(Some(self.num_threads()))
    }

    pub fn push_sample(&mut self, sample: Sample) {
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

fn frequency_dist(mut xs: Vec<f32>, num_ranges: usize) -> f32 {
    xs.sort_unstable_by(f32::total_cmp);

    let min = xs[0];
    let max = xs[xs.len() - 1];
    let dist_size = (max - min) / num_ranges as f32;
    let mut dist_max = (1..=num_ranges).map(|i| min + dist_size * i as f32).collect::<Vec<f32>>();
    dist_max[num_ranges - 1] = max;

    let mut dist = vec![Vec::new(); num_ranges];
    let mut dist_index = 0;
    for x in xs {
        while x > dist_max[dist_index] {
            dist_index += 1;
        }

        dist[dist_index].push(x);
    }

    let biggest_dist = dist.into_iter().max_by_key(Vec::len).unwrap();
    biggest_dist[0]
}
