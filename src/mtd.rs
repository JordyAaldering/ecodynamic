use crate::{controller::*, letterbox::Letterbox, sample::{Sample, SampleStart}};

pub struct Mtd {
    pub sample: SampleStart,
    letterbox: Letterbox,
    controller: Box<dyn Controller>,
    pub num_threads: f32,
}

impl Mtd {
    pub fn energy_controller(max_threads: usize, samples_per_update: usize) -> Self {
        Self {
            sample: SampleStart::new(),
            letterbox: Letterbox::new(samples_per_update),
            controller: Box::new(EnergyController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn runtime_controller(max_threads: usize) -> Self {
        Self {
            sample: SampleStart::new(),
            letterbox: Letterbox::new(20),
            controller: Box::new(RuntimeController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn fixed_controller(max_threads: usize) -> Self {
        Self {
            sample: SampleStart::new(),
            letterbox: Letterbox::new(1),
            controller: Box::new(FixedController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn install<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        self.sample.start();

        let res = f();

        let sample = self.sample.stop();
        self.update(sample);

        res
    }

    pub fn update(&mut self, sample: Sample) {
        if let Some(samples) = self.letterbox.push(sample) {
            self.num_threads = self.controller.adjust_threads(samples);
        }
    }

    pub fn num_threads(&self) -> i32 {
        self.num_threads.round() as i32
    }
}
