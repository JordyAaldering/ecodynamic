use crate::message::Demand;

use super::Controller;

pub struct OscilatingController {
    num_threads: i32,
    direction: i32,
    settings: OscilatingControllerSettings,
}

pub struct OscilatingControllerSettings {
    pub max_threads: i32,
}

impl OscilatingController {
    pub fn new(settings: OscilatingControllerSettings) -> Self {
        const DOWN: i32 = -1;
        Self {
            num_threads: settings.max_threads,
            direction: DOWN,
            settings,
        }
    }
}

impl Controller for OscilatingController {
    /// Interval depends on the number of samples; which is handled from the calling side
    fn evolve(&mut self, _scores: Vec<f32>) {
        self.num_threads += self.direction;

        // Swap direction if we are at an edge
        if self.num_threads <= 1 || self.num_threads >= self.settings.max_threads {
            self.direction = -self.direction;
        }
    }

    fn get_demand(&self) -> Demand {
        Demand { num_threads: self.num_threads }
    }
}
