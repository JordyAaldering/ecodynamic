use crate::message::Demand;

use super::Controller;

const _UP: i32 = 1;
const DOWN: i32 = -1;

pub struct CorridorController {
    max_threads: i32,
    num_threads: i32,
    step_size: i32,
    step_dir: i32,
    t_prev: f32,
    t1: f32,
}

impl CorridorController {
    pub fn new(max_threads: i32) -> Self {
        Self {
            max_threads,
            num_threads: max_threads,
            step_size: max_threads,
            step_dir: DOWN,
            t_prev: f32::MAX,
            t1: f32::MAX,
        }
    }
}

impl Controller for CorridorController {
    fn evolve(&mut self, scores: Vec<f32>) {
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
        self.num_threads = self.num_threads.max(1).min(self.max_threads);
    }

    fn next_demand(&mut self) -> Demand {
        Demand { num_threads: self.num_threads }
    }
}

fn frequency_dist(mut scores: Vec<f32>) -> f32 {
    const NUM_RANGES: usize = 5;

    scores.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let min = scores[0];
    let max = scores[scores.len() - 1];
    let dist_size = (max - min) / NUM_RANGES as f32;
    let mut dist_max = (1..=NUM_RANGES).map(|i| min + dist_size * i as f32).collect::<Vec<f32>>();
    dist_max[NUM_RANGES - 1] = max;

    let mut dist = vec![Vec::new(); NUM_RANGES];
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
