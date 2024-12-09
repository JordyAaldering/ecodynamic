use std::time::Instant;

use super::Controller;

const _UP: i32 = 1;
const DOWN: i32 = -1;

pub struct RuntimeController {
    num_threads: i32,
    max_threads: i32,
    t1: f32,
    t_prev: f32,
    step_size: i32,
    step_direction: i32,
}

impl RuntimeController {
    pub fn new(max_threads: i32) -> Self {
        Self {
            num_threads: max_threads,
            max_threads: max_threads,
            step_direction: DOWN,
            step_size: max_threads,
            t_prev: f32::MAX,
            t1: f32::MAX,
        }
    }
}

#[allow(unused)]
impl RuntimeController {
    fn sample_start() -> Instant {
        Instant::now()
    }

    fn sample_stop(sample: Instant) -> f32 {
        sample.elapsed().as_secs_f32()
    }
}

impl Controller for RuntimeController {
    fn adjust_threads(&mut self, samples: Vec<f32>) -> i32 {
        let tn = frequency_dist(samples);

        let speedup = self.t1 / tn;
        if speedup < 0.5 * self.num_threads as f32 {
            // We have fallen outside the corridor
            self.step_direction = DOWN;
            self.step_size = i32::max(1, self.num_threads as i32 / 2);
        } else {
            if speedup > self.num_threads as f32 {
                // In the initial iteration t1 and t_last are f64::MAX so we
                // reach this condition, an initialize t1 with an actual value
                self.t1 = tn * self.num_threads as f32;
            }

            if tn > self.t_prev {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.t_prev = tn;
        self.num_threads += self.step_direction * self.step_size;
        self.num_threads = self.num_threads.max(1).min(self.max_threads);
        self.num_threads
    }
}

const FREQDIST_RANGES: usize = 5;

fn frequency_dist(mut samples: Vec<f32>) -> f32 {
    samples.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let min = samples[0];
    let max = samples[samples.len() - 1];
    let dist_size = (max - min) / FREQDIST_RANGES as f32;
    let mut dist_max = (1..=FREQDIST_RANGES).map(|i| min + dist_size * i as f32).collect::<Vec<f32>>();
    dist_max[FREQDIST_RANGES - 1] = max;

    let mut dist = vec![Vec::new(); FREQDIST_RANGES];
    let mut dist_index = 0;
    for x in samples {
        while x > dist_max[dist_index] {
            dist_index += 1;
        }

        dist[dist_index].push(x);
    }

    let biggest_dist = dist.into_iter().max_by_key(Vec::len).unwrap();
    biggest_dist[0]
}
