use crate::{direction::Direction, Builder, Controller};

pub struct DeltaBuilder();

pub struct DeltaController {
    num_threads: f32,
    max_threads: f32,
    step_size: f32,
    step_direction: Direction,
    e_prev: f32,
}

impl Builder<DeltaController> for DeltaBuilder {
    fn build(&self, max_threads: i32) -> DeltaController {
        DeltaController {
            num_threads: max_threads as f32,
            max_threads: max_threads as f32,
            step_size: max_threads as f32,
            step_direction: Direction::Down,
            e_prev: 0.0,
        }
    }
}

impl DeltaController {
    /// Reset the step direction with a slight preference for increasing the thread count;
    /// since typically we don't want to end up in a case where we are single-threaded.
    fn reset_direction(&mut self) {
        self.step_direction = if self.num_threads < self.max_threads * 0.65 {
            Direction::Up
        } else {
            Direction::Down
        };
    }
}

impl Controller for DeltaController {
    fn adjust_threads(&mut self, samples: Vec<f32>) {
        let e_next = median(samples);

        if e_next > self.e_prev * 1.50 {
            self.step_size = self.max_threads * 0.5;
            self.reset_direction();
        } else {
            if e_next > self.e_prev {
                self.step_direction = -self.step_direction;
            }

            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_size = self.max_threads * 0.5;
                self.reset_direction();
            }
        }

        self.e_prev = e_next;
        self.num_threads += self.step_direction * self.step_size;
        self.num_threads = self.num_threads.max(1.0).min(self.max_threads);
    }

    fn get_threads(&self) -> i32 {
        self.num_threads.round() as i32
    }
}

fn median(mut vec: Vec<f32>) -> f32 {
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec[vec.len() / 2]
}
