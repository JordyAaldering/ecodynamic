use std::time::Instant;

use rapl_energy::{Energy, Rapl};

pub struct SampleStart {
    runtime: Instant,
    energy: Box<dyn Energy>,
}

pub struct Sample {
    pub runtime: f32,
    pub energy: f32,
}

impl SampleStart {
    pub fn new() -> Self {
        let runtime = Instant::now();
        let energy = Rapl::now().unwrap();
        Self { runtime, energy }
    }

    pub fn start(&mut self) {
        self.energy.reset();
        self.runtime = Instant::now();
    }

    pub fn stop(&self) -> Sample {
        let runtime = self.runtime.elapsed().as_secs_f32();
        let energy = self.energy.elapsed().into_values().sum();
        Sample { runtime, energy }
    }
}
