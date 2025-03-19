use crate::{dir::Direction, pct::Percentage, Controller};

pub struct CorridorController {
    num_threads: Percentage,
    step_size: Percentage,
    step_direction: Direction,
    t_prev: f32,
    t1: f32,
}

impl Default for CorridorController {
    fn default() -> Self {
        Self {
            num_threads: Percentage::FULL,
            step_size: Percentage::FULL,
            step_direction: Direction::Down,
            t_prev: f32::MAX,
            t1: f32::MAX,
        }
    }
}

impl Controller for CorridorController {
    fn adjust_threads(&mut self, samples: Vec<f32>) {
        let tn = frequency_dist(samples);

        if self.t1 / tn < 0.5 * *self.num_threads as f32 {
            // We have fallen outside the corridor
            self.step_size = Percentage::HALF;
            self.step_direction = Direction::Down;
        } else {
            if self.t1 / tn > *self.num_threads as f32 {
                // In the initial iteration t1 and t_prev are f32::MAX so we
                // reach this condition, an initialize t1 with an actual value
                self.t1 = tn * *self.num_threads as f32;
            }

            if tn > self.t_prev {
                self.step_direction = -self.step_direction;
            }

            self.step_size /= 2;
        }

        self.t_prev = tn;
        self.num_threads.adjust(self.step_size, self.step_direction);
    }

    fn num_threads(&self) -> u8 {
        *self.num_threads
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
