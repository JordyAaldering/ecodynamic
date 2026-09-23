use std::mem;

use ecodynamic_api::Sample;

pub struct Letterbox {
    size: usize,
    samples: Vec<Sample>,
}

impl Letterbox {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            samples: Vec::with_capacity(size),
        }
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Push a sample into the letterbox.
    /// If this fills the letterbox, returns the samples and clears the letterbox.
    pub fn push(&mut self, sample: Sample) -> Option<Vec<Sample>> {
        debug_assert!(self.samples.len() < self.size);

        self.samples.push(sample);

        if self.samples.len() == self.size {
            Some(mem::take(&mut self.samples))
        } else {
            None
        }
    }
}
