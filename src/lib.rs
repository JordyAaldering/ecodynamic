mod controller;
mod letterbox;

pub use controller::*;
pub use letterbox::Letterbox;

pub struct Mtd {
    letterbox: Letterbox,
    controller: Box<dyn Controller>,
    num_threads: i32,
}

impl Mtd {
    pub fn energy_controller(max_threads: i32) -> Self {
        Self {
            letterbox: Letterbox::new(10),
            controller: Box::new(EnergyController::new(max_threads)),
            num_threads: max_threads,
        }
    }

    pub fn runtime_controller(max_threads: i32) -> Self {
        Self {
            letterbox: Letterbox::new(20),
            controller: Box::new(RuntimeController::new(max_threads)),
            num_threads: max_threads,
        }
    }

    pub fn update(&mut self, sample: f32) {
        if let Some(samples) = self.letterbox.push(sample) {
            self.num_threads = self.controller.adjust_threads(samples);
        }
    }

    pub fn num_threads(&self) -> i32 {
        self.num_threads
    }
}
