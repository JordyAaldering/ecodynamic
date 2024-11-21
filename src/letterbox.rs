use crate::sample::Sample;

pub struct Letterbox {
    size: usize,
    samples: Option<Vec<Sample>>,
}

impl Letterbox {
    pub fn new(size: usize) -> Self {
        Self { size, samples: None }
    }

    pub fn push(&mut self, sample: Sample) -> Option<Vec<Sample>> {
        let samples = self.samples.get_or_insert_with(|| Vec::with_capacity(self.size));
        samples.push(sample);

        if samples.len() >= self.size {
            self.samples.take()
        } else {
            None
        }
    }
}
