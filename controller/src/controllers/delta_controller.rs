use crate::{Controller, Demand, Sample};

const UP: f32 = 1.0;
const DOWN: f32 = -1.0;

pub struct DeltaController {
    num_threads: f32,
    max_threads: f32,
    step_size: f32,
    step_dir: f32,
    e_prev: f32,
}

impl DeltaController {
    pub fn new(max_threads: i32) -> Self {
        Self {
            num_threads: max_threads as f32,
            max_threads: max_threads as f32,
            step_size: max_threads as f32,
            step_dir: DOWN,
            e_prev: 0.0,
        }
    }
}

impl Controller for DeltaController {
    fn update(&mut self, samples: Vec<Sample>) {
        let samples = samples.into_iter().map(|s| s.runtime).collect();
        let e_next = median(samples);

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

    fn next(&mut self) -> Demand {
        Demand { num_threads: self.num_threads.round() as i32 }
    }
}

impl DeltaController {
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

fn median(mut vec: Vec<f32>) -> f32 {
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec[vec.len() / 2]
}
