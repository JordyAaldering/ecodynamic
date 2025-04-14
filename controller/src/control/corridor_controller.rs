use crate::message::{Demand, Sample};

use super::Controller;

const _UP: i32 = 1;
const DOWN: i32 = -1;

pub struct CorridorController {
    samples: Vec<Sample>,
    num_threads: i32,
    step_size: i32,
    step_dir: i32,
    t_prev: f32,
    t1: f32,
    settings: CorridorControllerSettings,
}

pub struct CorridorControllerSettings {
    pub max_threads: i32,
    pub score_fn: fn(Sample) -> f32,
    pub population_size: usize,
}

impl CorridorController {
    pub fn new(settings: CorridorControllerSettings) -> Self {
        Self {
            samples: Vec::new(),
            num_threads: settings.max_threads,
            step_size: settings.max_threads,
            step_dir: DOWN,
            t_prev: f32::MAX,
            t1: f32::MAX,
            settings,
        }
    }

    fn evolve(&mut self) {
        let mut samples_new = Vec::new();
        std::mem::swap(&mut self.samples, &mut samples_new);
        let scores = samples_new.into_iter().map(self.settings.score_fn).collect();
        let tn = frequency_dist(scores);

        if self.t1 / tn < 0.5 * self.num_threads as f32 {
            self.step_dir = DOWN;
            self.step_size = i32::max(1, self.num_threads / 2);
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
        self.num_threads = self.num_threads.max(1).min(self.settings.max_threads);
    }
}

impl Controller for CorridorController {
    fn sample_received(&mut self, sample: Sample) {
        self.samples.push(sample);
        if self.samples.len() >= self.settings.population_size {
            self.evolve();
        }
    }

    fn next_demand(&mut self) -> Demand {
        Demand { num_threads: self.num_threads }
    }
}

const FREQDIST_RANGES: usize = 5;

fn frequency_dist(mut scores: Vec<f32>) -> f32 {
    scores.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let min = scores[0];
    let max = scores[scores.len() - 1];
    let dist_size = (max - min) / FREQDIST_RANGES as f32;
    let mut dist_max = (1..=FREQDIST_RANGES).map(|i| min + dist_size * i as f32).collect::<Vec<f32>>();
    dist_max[FREQDIST_RANGES - 1] = max;

    let mut dist = vec![Vec::new(); FREQDIST_RANGES];
    let mut dist_index = 0;
    for x in scores {
        while x > dist_max[dist_index] {
            dist_index += 1;
        }

        dist[dist_index].push(x);
    }

    let biggest_dist = dist.into_iter().max_by_key(Vec::len).unwrap();
    biggest_dist[0]
}
