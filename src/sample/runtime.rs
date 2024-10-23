use std::time::Instant;

use super::Sample;

pub struct SampleRuntime {
    start: Instant,
}

impl SampleRuntime {
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }
}

impl Sample for SampleRuntime {
    fn start(&mut self) {
        self.start = Instant::now();
    }

    fn stop(&self) -> f32 {
        self.start.elapsed().as_secs_f32()
    }
}
