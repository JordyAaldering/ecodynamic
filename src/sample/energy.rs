use rapl_energy::{Energy, Rapl};

use super::Sample;

pub struct SampleEnergy {
    rapl: Box<dyn Energy>,
}

impl SampleEnergy {
    pub fn new() -> Self {
        let rapl = Rapl::now().unwrap();
        Self { rapl }
    }
}

impl Sample for SampleEnergy {
    fn start(&mut self) {
        self.rapl.reset();
    }

    fn stop(&self) -> f32 {
        self.rapl.elapsed().into_values().sum()
    }
}
