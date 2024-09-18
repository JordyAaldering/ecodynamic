#[derive(Clone)]
pub struct Sample {
    runtime: f64,
    usertime: f64,
    energy: f64,
    num_threads: i32,
}

impl Sample {
    pub fn new(runtime: f64, usertime: f64, energy: f64) -> Self {
        Sample { runtime, usertime, energy, num_threads: -1 }
    }

    pub fn energy_score(&self) -> f64 {
        self.energy
    }
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {}, {})", self.runtime, self.usertime, self.energy, self.num_threads))
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
        let len = self.history.len() as f64;
        let real_time = self.history.iter().map(|sample| sample.runtime).sum::<f64>() / len;
        let user_time = self.history.iter().map(|sample| sample.usertime).sum::<f64>() / len;
        let energy = self.history.iter().map(|sample| sample.energy).sum::<f64>() / len;
        f.write_fmt(format_args!("{},{},{}", real_time, user_time, energy))
    }
}
