mod direction;
mod selection;

use crate::{clamp::Clamp, letterbox::Sample};
use direction::Direction;
use selection::*;

const NUM_BUCKETS: usize = 5;

pub struct Controller {
    n: Clamp<i32>,
    step_size: Clamp<i32>,
    step_direction: Direction,
    corridor_width: f32,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
    t_best_buckets: Vec<(u64, i32)>,
    t_last: u64,
}

fn user_frac(sample: &Sample) -> f32 {
    sample.usertime_ns as f32 / sample.realtime_ns as f32
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: Clamp::new(max_threads, 1, max_threads),
            step_size: Clamp::new(max_threads, 1, max_threads),
            step_direction: Direction::Down,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
            t_best_buckets: vec![(u64::MAX, max_threads); NUM_BUCKETS],
            t_last: u64::MAX,
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
                self.step_direction = Direction::towards(self.n.get(), t_best_thread_count);

                let diff = i32::abs(self.n.get() - t_best_thread_count);
                self.step_size.set(diff / 2);
            }

            println!("Fallen outside the corridor (speedup = {}), step size to {}", speedup, self.step_size);
        } else {
            if speedup > 1.0 / (1.0 - self.corridor_width) {
                println!("Went above the corridor (speedup = {}), step size to {}", speedup, self.n.get() / 2);
                self.step_size.set(self.n.get()); // Will be n / 2 at the end of this block
            }

            if tn < t_best {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("T_best[{}] updated to {} at {} threads", bucket, tn, self.n);
                self.t_best_buckets[bucket] = (tn, self.n.get());
            }

            if tn > self.t_last {
                // The previous iteration performed better; reverse direction
                self.step_direction = -self.step_direction;
            }

            self.step_size /= 2;
        }

        self.t_last = tn;

        self.n += self.step_direction * self.step_size.get();
        self.n.get()
    }

    fn get_bucket(&self, samples: &Vec<Sample>) -> usize {
        let user_frac: f32 = samples.iter().map(user_frac).sum();
        let user_frac = user_frac / samples.len() as f32;
        usize::min((user_frac * NUM_BUCKETS as f32) as usize, NUM_BUCKETS - 1)
    }
}
