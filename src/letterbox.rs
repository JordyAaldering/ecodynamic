pub struct Letterbox {
    runtimes: Option<Vec<u64>>,
    pub num_threads: i32,
}

impl Letterbox {
    pub fn new(max_threads: i32) -> Self {
        Letterbox {
            runtimes: None,
            num_threads: max_threads,
        }
    }

    pub fn push(&mut self, value: u64) -> usize {
        if let Some(vec) = &mut self.runtimes {
            vec.push(value);
            vec.len()
        } else {
            let vec = vec![value];
            self.runtimes = Some(vec);
            1
        }
    }

    pub fn take(&mut self) -> Vec<u64> {
        self.runtimes.take().unwrap()
    }
}
