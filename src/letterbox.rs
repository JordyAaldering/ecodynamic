pub struct Letterbox {
    pub samples: Option<Vec<(u64, u64)>>,
    pub num_threads: i32,
}

impl Letterbox {
    pub fn new(max_threads: i32) -> Self {
        Letterbox {
            samples: None,
            num_threads: max_threads,
        }
    }

    pub fn push(&mut self, runtime: u64, energy: u64) -> usize {
        if let Some(vec) = &mut self.samples {
            vec.push((runtime, energy));
            vec.len()
        } else {
            let vec = vec![(runtime, energy)];
            self.samples = Some(vec);
            1
        }
    }

    pub fn take(&mut self) -> Vec<(u64, u64)> {
        self.samples.take().unwrap()
    }
}
