use std::time::Instant;

use rapl_energy::{Energy, Rapl};

pub struct Sample {
    runtime: Instant,
    energy: Box<dyn Energy>,
}

impl Sample {
    pub fn new() -> Self {
        let runtime = Instant::now();
        let energy = Rapl::now().unwrap();
        Self { runtime, energy }
    }

    pub fn start(&mut self) {
        self.energy.reset();
        self.runtime = Instant::now();
    }

    pub fn stop(&self) -> (f32, f32) {
        let runtime = self.runtime.elapsed().as_secs_f32();
        let energy = self.energy.elapsed().into_values().sum();
        (runtime, energy)
    }
}
