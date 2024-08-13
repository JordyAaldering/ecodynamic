pub struct Sample {
    realtime_ns: u64,
    usertime_ns: u64,
    energy_uj: u64,
}

pub struct Letterbox {
    samples: Option<Vec<(u64, u64, u64)>>,
    pub num_threads: i32,
    // Debugging
    pub history: Vec<(u64, u64, u64)>,
}

impl Letterbox {
    pub fn new(max_threads: i32) -> Self {
        Letterbox {
            samples: None,
            num_threads: max_threads,
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, realtime: u64, usertime: u64, energy: u64) -> usize {
        self.history.push((realtime, usertime, energy));

        if let Some(vec) = &mut self.samples {
            vec.push((realtime, usertime, energy));
            vec.len()
        } else {
            let vec = vec![(realtime, usertime, energy)];
            self.samples = Some(vec);
            1
        }
    }

    pub fn take(&mut self) -> Vec<(u64, u64, u64)> {
        self.samples.take().unwrap()
    }
}
