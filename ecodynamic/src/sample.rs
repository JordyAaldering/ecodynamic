use std::time::Instant;

use controller::Sample;
use rapl_energy::Rapl;

pub struct SampleInstant {
    pub time: Instant,
    pub rapl: Rapl,
}

impl SampleInstant {
    pub fn start() -> Self {
        Self {
            rapl: Rapl::now(false).unwrap(),
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
            usertime: 0.0,
        }
    }
}
