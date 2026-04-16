use std::time::Instant;

use controller::Sample;
use rapl_energy::Rapl;

pub struct SamplePair {
    pub time: Instant,
    pub rapl: Rapl,
}

impl SamplePair {
    pub fn start() -> Self {
        Self {
            rapl: Rapl::new(false).unwrap(),
            time: Instant::now(),
        }
    }

    pub fn stop(self, region_uid: i32) -> Sample {
        let runtime =self.time.elapsed();
        let energy = self.rapl.elapsed();
        Sample {
            region_uid,
            runtime: runtime.as_secs_f32(),
            energy: energy.values().sum(),
            usertime: None,
        }
    }
}
