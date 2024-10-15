#[derive(Clone)]
pub struct Sample {
    pub runtime: f64,
    pub usertime: f64,
    pub energy: f64,
    num_threads: f64,
}

impl Sample {
    pub fn new(runtime: f64, usertime: f64, energy: f64) -> Self {
        Sample { runtime, usertime, energy, num_threads: -1.0 }
    }
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{},{},{}", self.runtime, self.usertime, self.energy, self.num_threads))
    }
}

pub struct Letterbox {
    samples: Option<Vec<Sample>>,
    pub num_threads: f64,
    pub num_measurements_per_adjustment: usize,
    // Debugging and analysis
    pub history: Vec<Sample>,
}

impl Letterbox {
    pub fn new(max_threads: f64, num_measurements_per_adjustment: usize) -> Self {
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

    pub fn take(&mut self) -> Vec<Sample> {
        self.samples.take().unwrap()
    }
}

impl std::fmt::Debug for Letterbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let energy: f64 = self.history.iter().map(|x| x.energy).sum();
        let realtime: f64 = self.history.iter().map(|x| x.runtime).sum();
        let usertime: f64 = self.history.iter().map(|x| x.usertime).sum();
        f.write_fmt(format_args!("{},{},{}", energy, realtime, usertime))
    }
}
