#[derive(Clone)]
pub struct Sample {
    pub realtime_ns: u64,
    pub usertime_ns: u64,
    pub energy_uj: u64,
}

impl Sample {
    pub fn new(realtime_ns: u64, usertime_ns: u64, energy_uj: u64) -> Self {
        Sample { realtime_ns, usertime_ns, energy_uj }
    }
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.realtime_ns, self.usertime_ns, self.energy_uj))
    }
}

pub struct Letterbox {
    samples: Option<Vec<Sample>>,
    pub num_threads: i32,
    // Debugging
    pub history: Vec<Sample>,
}

impl Letterbox {
    pub fn new(max_threads: i32) -> Self {
        Letterbox {
            samples: None,
            num_threads: max_threads,
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, sample: Sample) -> usize {
        self.history.push(sample.clone());

        if let Some(vec) = &mut self.samples {
            vec.push(sample);
            vec.len()
        } else {
            let vec = vec![sample];
            self.samples = Some(vec);
            1
        }
    }

    pub fn take(&mut self) -> Vec<Sample> {
        self.samples.take().unwrap()
    }
}
