#[derive(Clone)]
pub struct Sample {
    pub realtime_ns: u64,
    pub usertime_ns: u64,
    pub energy_uj: u64,
    num_threads: i32,
}

impl Sample {
    pub fn new(realtime_ns: u64, usertime_ns: u64, energy_uj: u64) -> Self {
        Sample { realtime_ns, usertime_ns, energy_uj, num_threads: -1 }
    }
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {}, {})", self.realtime_ns, self.usertime_ns, self.energy_uj, self.num_threads))
    }
}

pub struct Letterbox {
    samples: Option<Vec<Sample>>,
    num_threads: i32,
    num_measurements_per_adjustment: usize,
    // Debugging and analysis
    history: Vec<Sample>,
}

impl Letterbox {
    pub fn new(max_threads: i32, num_measurements_per_adjustment: usize) -> Self {
        Letterbox {
            samples: None,
            num_threads: max_threads,
            num_measurements_per_adjustment,
            // Debugging and analysis
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, mut sample: Sample) -> usize {
        sample.num_threads = self.num_threads;
        self.history.push(sample.clone());

        if let Some(vec) = &mut self.samples {
            vec.push(sample);
            vec.len()
        } else {
            let mut vec = Vec::with_capacity(self.num_measurements_per_adjustment);
            vec.push(sample);
            self.samples = Some(vec);
            1
        }
    }

    pub fn update_threads(&mut self, num_threads: i32) {
        self.num_threads = num_threads;
    }

    pub fn num_threads(&self) -> i32 {
        self.num_threads
    }

    pub fn take(&mut self) -> Vec<Sample> {
        self.samples.take().unwrap()
    }
}

impl std::fmt::Debug for Letterbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let real_time: u64 = self.history.iter().map(|sample| sample.realtime_ns).sum();
        let user_time: u64 = self.history.iter().map(|sample| sample.usertime_ns).sum();
        let energy: u64 = self.history.iter().map(|sample| sample.energy_uj).sum();
        f.write_fmt(format_args!(
            "{:?}\n\tReal time ms: {}, user time ms: {}, energy mJ: {}",
            self.history,
            real_time / 1_000_000,
            user_time / 1_000_000,
            energy / 1_000))
    }
}
