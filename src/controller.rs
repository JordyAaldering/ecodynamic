mod direction;
mod selection;

use crate::letterbox::Sample;
use direction::Direction;
use selection::*;

const NUM_BUCKETS: usize = 5;

pub struct Controller {
    n: i32,
    t_best_buckets: Vec<(u64, i32)>,
    t_last: u64,
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f32,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

fn user_frac(sample: &Sample) -> f32 {
    sample.usertime_ns as f32 / sample.realtime_ns as f32
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: max_threads,
            //t_best: u64::MAX,
            //t_best_thread_count: max_threads as u64,
            t_best_buckets: vec![(u64::MAX, max_threads); NUM_BUCKETS],
            t_last: u64::MAX,
            step_size: max_threads,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let bucket = self.get_bucket(&samples);
        let (t_best, t_best_thread_count) = self.t_best_buckets[bucket];

        let samples = samples.iter().map(|sample| sample.energy_uj).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = t_best as f32 / tn as f32;

        if speedup < 1.0 - self.corridor_width {
            // Fallen outside the corridor
            // Move up or down depending on where the best thread count was
            if tn > self.t_last {
                // The previous iteration performed much better; reverse direction
                self.step_direction = -self.step_direction;

                self.step_size *= 2;
            } else {
                // Otherwise we move towards our estimated optimum
                self.step_direction = Direction::towards(self.n, t_best_thread_count);

                let diff = i32::abs(self.n - t_best_thread_count);
                self.step_size = diff / 2;
            }

            println!("Fallen outside the corridor (speedup = {}), step size to {}", speedup, self.step_size);
        } else {
            if speedup > 1.0 / (1.0 - self.corridor_width) {
                println!("Went above the corridor (speedup = {}), step size to {}", speedup, self.n / 2);
                self.step_size = self.n; // Will be n / 2 at the end of this block
            }

            if tn < t_best {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("T_best[{}] updated to {} at {} threads", bucket, tn, self.n);
                self.t_best_buckets[bucket] = (tn, self.n);
            }

            if tn > self.t_last {
                // The previous iteration performed better; reverse direction
                self.step_direction = -self.step_direction;
            }

            self.step_size /= 2;
        }

        self.t_last = tn;

        self.step_size = i32::max(self.step_size, 1);
        self.n += self.step_direction * self.step_size;
        self.n = i32::min(self.n, self.max_threads);
        self.n = i32::max(self.n, 1);
        self.n
    }

    fn get_bucket(&self, samples: &Vec<Sample>) -> usize {
        let user_frac: f32 = samples.iter().map(user_frac).sum();
        let user_frac = user_frac / samples.len() as f32;
        usize::min((user_frac * NUM_BUCKETS as f32) as usize, NUM_BUCKETS - 1)
    }
}
