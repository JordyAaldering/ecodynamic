use std::collections::HashMap;

use controller::{Builder, Controller};

use crate::{Demand, Sample};

pub struct Letterbox<Ctrl: Controller, const N: usize> {
    builder: Box<dyn Builder<Ctrl>>,
    letterbox: HashMap<i32, (Ctrl, Samples<N>)>,
}

impl<Ctrl: Controller, const N: usize> Letterbox<Ctrl, N> {
    pub fn new(builder: Box<dyn Builder<Ctrl>>) -> Self {
        Self { builder, letterbox: HashMap::new() }
    }

    pub fn update(&mut self, sample: Sample) -> Demand {
        let threads = if let Some((controller, samples)) = self.letterbox.get_mut(&sample.uid) {
            samples.push(sample.val);

            if samples.len >= N {
                controller.adjust_threads(samples.take())
            }

            controller.get_threads()
        } else {
            let controller = self.builder.build(sample.max);
            let samples = Samples::from(sample.val);
            self.letterbox.insert(sample.uid, (controller, samples));
            sample.max
        };

        Demand { threads }
    }
}

struct Samples<const N: usize> {
    elems: [f32; N],
    len: usize,
}

impl<const N: usize> Samples<N> {
    fn take(&mut self) -> Vec<f32> {
        self.len = 0;
        self.elems.to_vec()
    }

    fn push(&mut self, value: f32) {
        self.elems[self.len] = value;
        self.len += 1;
    }
}

impl<const N: usize> From<f32> for Samples<N> {
    fn from(value: f32) -> Self {
        let mut elems = [0f32; N];
        elems[0] = value;
        Self { elems, len: 1 }
    }
}
