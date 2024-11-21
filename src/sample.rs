use std::time::Instant;

use rapl_energy::{Energy, Rapl};

pub struct SampleInstant {
    runtime: Instant,
    energy: Box<dyn Energy>,
}

#[derive(Clone)]
pub struct Sample {
    pub runtime: f32,
    pub energy: f32,
}

impl SampleInstant {
    pub fn now() -> Self {
        let energy = Rapl::now().unwrap();
        let runtime = Instant::now();
        Self { runtime, energy }
    }

    pub fn elapsed(&self) -> Sample {
        let runtime = self.runtime.elapsed().as_secs_f32();
        let energy = self.energy.elapsed().into_values().sum();
        Sample { runtime, energy }
    }
}
