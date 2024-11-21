use std::time::Instant;

use rapl_energy::{Energy, Rapl};

pub struct SampleInstant {
    now: Instant,
    rapl: Rapl,
}

#[derive(Clone)]
pub struct Sample {
    pub runtime: f32,
    pub energy: f32,
}

impl SampleInstant {
    pub fn now() -> Self {
        let rapl = Rapl::now().unwrap();
        let now = Instant::now();
        Self { now, rapl }
    }

    pub fn elapsed(&self) -> Sample {
        let runtime = self.now.elapsed().as_secs_f32();
        let energy = self.rapl.elapsed().into_values().sum();
        Sample { runtime, energy }
    }
}
