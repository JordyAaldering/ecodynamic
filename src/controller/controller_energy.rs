use super::{direction::Direction, thread_count::ThreadCount, Controller};

pub struct EnergyController {
    num_threads: ThreadCount,
    max_threads: f32,
    step_size: f32,
    step_direction: Direction,
    e_prev: f32,
}

impl EnergyController {
    pub fn new(max_threads: usize) -> Self {
        Self {
            num_threads: ThreadCount::new(max_threads),
            max_threads: max_threads as f32,
            step_size: max_threads as f32,
            step_direction: Direction::Down,
            e_prev: 0.0,
        }
    }
}

impl Controller for EnergyController {
    fn adjust_threads(&mut self, e_avg: f32) -> f32 {
        let e_avg = round_digits(e_avg, 3);
        if e_avg > self.e_prev * 1.50 {
            // Previous iteration performed a lot better
            self.step_direction = towards_farthest_edge(*self.num_threads, self.max_threads);
            self.step_size = self.max_threads * 0.5;
        } else {
            if e_avg > self.e_prev {
                // Previous iteration performed (a bit) better
                self.step_direction = -self.step_direction;
            }

            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_direction = towards_farthest_edge(*self.num_threads, self.max_threads);
                self.step_size = self.max_threads * 0.5;
            }
        }

        self.e_prev = e_avg;
        self.num_threads += self.step_direction * self.step_size;
        *self.num_threads as f32
    }
}

#[inline]
fn towards_farthest_edge(n: f32, max_threads: f32) -> Direction {
    // Slight preference for increasing the thread count; since typically we
    // don't want to end up in a case where we are running single-threaded
    if n < max_threads * 0.65 {
        Direction::Up
    } else {
        Direction::Down
    }
}

#[inline]
fn round_digits(x: f32, decimals: u32) -> f32 {
    let y = 10u32.pow(decimals) as f32;
    (x * y).round() / y
}
