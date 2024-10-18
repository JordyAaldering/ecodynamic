use crate::{controller::{Controller, FrequencyDist, SelectionAlgorithm}, controller_energy::EnergyController, controller_runtime::RuntimeController, letterbox::Letterbox};

pub struct Mtd {
    letterbox: Letterbox,
    selection: FrequencyDist,
    controller: Box<dyn Controller>,
    pub num_threads: f32,
}

impl Mtd {
    pub fn energy_controller(max_threads: i32) -> Self {
        Self {
            letterbox: Letterbox::new(10),
            selection: FrequencyDist::new(4, true),
            controller: Box::new(EnergyController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn runtime_controller(max_threads: i32) -> Self {
        Self {
            letterbox: Letterbox::new(20),
            selection: FrequencyDist::new(5, false),
            controller: Box::new(RuntimeController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn update(&mut self, sample: f32) {
        if let Some(samples) = self.letterbox.push(sample) {
            let samples = samples.into_iter().map(f64::from).collect();
            let tn = self.selection.find_best(samples) as f32;
            self.num_threads = self.controller.adjust_threads(tn);
        }
    }

    pub fn num_threads(&self) -> i32 {
        self.num_threads.round() as i32
    }
}
