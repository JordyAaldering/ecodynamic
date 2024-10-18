pub struct Letterbox {
    size: usize,
    samples: Option<Vec<f32>>,
}

impl Letterbox {
    pub fn new(size: usize) -> Self {
        Self { size, samples: None }
    }

    pub fn push(&mut self, sample: f32) -> Option<Vec<f32>> {
        let samples = self.samples.get_or_insert_with(|| Vec::with_capacity(self.size));
        samples.push(sample);

        if samples.len() >= self.size {
            self.samples.take()
        } else {
            None
        }
    }
}
