use std::mem;

use controller::Sample;

use crate::CONFIG;

pub struct SampleVec {
    inner: Vec<Sample>,
}

impl SampleVec {
    pub fn new() -> Self {
        let capacity = CONFIG.lock().letterbox_size;
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn push_until_full(&mut self, sample: Sample) -> Option<Vec<Sample>> {
        self.inner.push(sample);

        let capacity = CONFIG.lock().letterbox_size;

        if self.inner.len() >= capacity {
            let mut swap = Vec::with_capacity(capacity);
            mem::swap(&mut self.inner, &mut swap);
            Some(swap)
        } else {
            None
        }
    }
}
