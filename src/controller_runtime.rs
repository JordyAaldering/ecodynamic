use crate::controller::{Controller, Direction};

pub struct RuntimeController {
    n: usize,
    t1: f32,
    t_last: f32,
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: usize,
    corridor_width: f32,
}

impl RuntimeController {
    pub fn new(max_threads: usize) -> Self {
        Self {
            n: max_threads,
            t1: f32::MAX,
            t_last: f32::MAX,
            step_size: max_threads as i32,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
        }
    }
}

impl Controller for RuntimeController {
    fn adjust_threads(&mut self, tn: f32) -> f32 {
        let speedup = self.t1 / tn;
        if speedup < (1.0 - self.corridor_width) * self.n as f32 {
            // We have fallen outside the corridor
            self.step_direction = Direction::Down;
            self.step_size = i32::max(1, self.n as i32 / 2);
        } else {
            if speedup > self.n as f32 {
                // In the initial iteration t1 and t_last are f64::MAX so we
                // reach this condition, an initialize t1 with an actual value
                self.t1 = tn * self.n as f32;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.t_last = tn;
        self.n += (self.step_direction * self.step_size as f32) as usize;
        self.n = usize::max(1, usize::min(self.max_threads, self.n));
        self.n as f32
    }
}
