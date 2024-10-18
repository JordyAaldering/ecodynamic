use crate::controller::{Controller, Direction, ThreadCount};

pub struct EnergyController {
    n: ThreadCount,
    max_threads: f32,
    step_size: f32,
    step_direction: Direction,
    t_last: f32,
}

impl EnergyController {
    pub fn new(max_threads: usize) -> Self {
        Self {
            n: ThreadCount::new(max_threads),
            max_threads: max_threads as f32,
            step_size: max_threads as f32,
            step_direction: Direction::Down,
            t_last: 0.0,
        }
    }
}

impl Controller for EnergyController {
    fn adjust_threads(&mut self, tn: f32) -> f32 {
        if tn > self.t_last * 1.50 {
            // Previous iteration performed a lot better
            self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
            self.step_size = self.max_threads * 0.5;
        } else {
            if tn > self.t_last * 1.02 {
                // Previous iteration performed (a bit) better
                self.step_direction = -self.step_direction;
            }

            if self.step_size > 0.16 {
                self.step_size = f32::max(self.step_size * 0.6, self.step_size / (0.85 + self.step_size));
            } else {
                self.step_direction = towards_farthest_edge(*self.n, self.max_threads);
                self.step_size = self.max_threads * 0.5;
            }
        }

        if tn < self.t_last || tn > self.t_last * 1.02 {
            // Only update after a significant change
            self.t_last = tn;
        }

        self.n += self.step_direction * self.step_size;
        *self.n as f32
    }
}

#[inline]
fn towards_farthest_edge(n: f32, max_threads: f32) -> Direction {
    // Prefer to move up; typically we don't want to end up in a case where we are running single-threaded
    if n > max_threads * 0.65 {
        Direction::Down
    } else {
        Direction::Up
    }
}
