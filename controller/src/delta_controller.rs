use crate::{dir::Direction, pct::Percentage, Controller};

pub struct DeltaController {
    num_threads: Percentage,
    step_size: Percentage,
    step_direction: Direction,
    e_prev: f32,
}

impl Default for DeltaController {
    fn default() -> Self {
        Self {
            num_threads: Percentage::FULL,
            step_size: Percentage::FULL,
            step_direction: Direction::Down,
            e_prev: 0.0,
        }
    }
}

impl Controller for DeltaController {
    fn adjust_threads(&mut self, samples: Vec<f32>) {
        let e_next = median(samples);

        if e_next > self.e_prev * 1.50 {
            self.step_size = Percentage::HALF;
            self.reset_direction();
        } else {
            if e_next > self.e_prev {
                self.step_direction = -self.step_direction;
            }

            if *self.step_size > 16 {
                // todo, obviously we need to clean this up
                self.step_size.map(|x| f32::max(x as f32 * 0.6, x as f32 / (0.85 + x as f32)) as u8);
            } else {
                self.step_size = Percentage::HALF;
                self.reset_direction();
            }
        }

        self.e_prev = e_next;
        self.num_threads.adjust(self.step_size, self.step_direction);
    }

    fn num_threads(&self) -> u8 {
        *self.num_threads
    }
}

impl DeltaController {
    /// Reset the step direction with a slight preference for increasing the thread count;
    /// since typically we don't want to end up in a case where we are single-threaded.
    fn reset_direction(&mut self) {
        self.step_direction = if *self.num_threads < 65 {
            Direction::Up
        } else {
            Direction::Down
        };
    }
}

fn median(mut vec: Vec<f32>) -> f32 {
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec[vec.len() / 2]
}
