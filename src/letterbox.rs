use crate::controller::FrequencyDist;

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
    pub history: Vec<Sample>,
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
        let freq = FrequencyDist::new(5);

        let energy = self.history.iter().map(|sample| sample.energy).collect();
        let distributions = freq.distribute(energy);
        let energy_dist = distributions.into_iter().max_by_key(Vec::len).unwrap();
        let energy_len = energy_dist.len() as f64;
        let energy = energy_dist.into_iter().sum::<f64>() / energy_len;

        let real_time = self.history.iter().map(|sample| sample.runtime).collect();
        let distributions = freq.distribute(real_time);
        let real_time_dist = distributions.into_iter().max_by_key(Vec::len).unwrap();
        let real_time_len = real_time_dist.len() as f64;
        let real_time = real_time_dist.into_iter().sum::<f64>() / real_time_len;

        let user_time = self.history.iter().map(|sample| sample.usertime).collect();
        let distributions = freq.distribute(user_time);
        let user_time_dist = distributions.into_iter().max_by_key(Vec::len).unwrap();
        let user_time_len = user_time_dist.len() as f64;
        let user_time = user_time_dist.into_iter().sum::<f64>() / user_time_len;

        f.write_fmt(format_args!("{},{},{}", real_time, user_time, energy))
    }
}
