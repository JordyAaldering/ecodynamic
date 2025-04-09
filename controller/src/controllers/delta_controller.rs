use crate::{Controller, Demand, Sample};

const UP: f32 = 1.0;
const DOWN: f32 = -1.0;

pub struct DeltaController {
    samples: Vec<Sample>,
    num_threads: f32,
    max_threads: f32,
    step_size: f32,
    step_dir: f32,
    e_prev: f32,
}

impl DeltaController {
    pub fn new(max_threads: i32) -> Self {
        Self {
            samples: Vec::new(),
            num_threads: max_threads as f32,
            max_threads: max_threads as f32,
            step_size: max_threads as f32,
            step_dir: DOWN,
            e_prev: 0.0,
        }
    }

    fn evolve(&mut self) {
        let mut samples_new = Vec::new();
        std::mem::swap(&mut self.samples, &mut samples_new);
        let e_next = median(samples_new);

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
    fn sample_received(&mut self, sample: Sample) {
        self.samples.push(sample);
        if self.samples.len() >= 10 {
            self.evolve();
        }
    }

    fn next_demand(&mut self) -> Demand {
        let num_threads = self.num_threads.round() as i32;
        Demand { num_threads }
    }
}

fn median(samples: Vec<Sample>) -> f32 {
    let mut scores: Vec<f32> = samples.into_iter().map(|s| s.energy).collect();
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    scores[scores.len() / 2]
}
