use crate::message::Demand;

use super::Controller;

pub struct OscilatingController {
    num_threads: i32,
    direction: i32,
    iteration: usize,
    settings: OscilatingControllerSettings,
}

pub struct OscilatingControllerSettings {
    pub max_threads: i32,
    pub interval: usize,
}

impl OscilatingController {
    pub fn new(settings: OscilatingControllerSettings) -> Self {
        const DOWN: i32 = -1;
        Self {
            num_threads: settings.max_threads,
            direction: DOWN,
            iteration: 0,
            settings,
        }
    }
}

impl Controller for OscilatingController {
    fn sample_received(&mut self, _score: f32) { }

    fn next_demand(&mut self) -> Demand {
        self.iteration += 1;

        // Update demand every N iterations
        if self.iteration >= self.settings.interval {
            self.num_threads += self.direction;
            // Swap direction if we are at an edge
            if self.num_threads <= 1 || self.num_threads >= self.settings.max_threads {
                self.direction = -self.direction;
            }

            self.iteration = 0;
        }

        Demand { num_threads: self.num_threads }
    }
}
