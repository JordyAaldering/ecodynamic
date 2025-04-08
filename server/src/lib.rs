use std::collections::HashMap;

use controller::{Builder, Controller, Demand, Sample};

pub const MTD_LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

pub struct Letterbox<Ctrl: Controller, const N: usize> {
    builder: Box<dyn Builder<Ctrl>>,
    letterbox: HashMap<i32, (Ctrl, Samples<N>)>,
}

impl<Ctrl: Controller, const N: usize> Letterbox<Ctrl, N> {
    pub fn new(builder: Box<dyn Builder<Ctrl>>) -> Self {
        Self { builder, letterbox: HashMap::new() }
    }

    pub fn update(&mut self, sample: Sample) -> Demand {
        let threads = if let Some((controller, samples)) = self.letterbox.get_mut(&sample.region_uid) {
            samples.push(sample);

            if samples.len >= N {
                let samples = samples.take().into_iter().map(|s| s.energy).collect();
                controller.adjust_threads(samples)
            }

            controller.num_threads()
        } else {
            let uid = sample.region_uid;
            let max_threads = sample.max_threads;
            let controller = self.builder.build(max_threads);
            let samples = Samples::from(sample);
            self.letterbox.insert(uid, (controller, samples));
            max_threads
        };

        Demand { num_threads: threads }
    }
}

struct Samples<const N: usize> {
    elems: [Sample; N],
    len: usize,
}

impl<const N: usize> Samples<N> {
    fn take(&mut self) -> Vec<Sample> {
        self.len = 0;
        self.elems.to_vec()
    }

    fn push(&mut self, value: Sample) {
        self.elems[self.len] = value;
        self.len += 1;
    }
}

impl<const N: usize> From<Sample> for Samples<N> {
    fn from(value: Sample) -> Self {
        const SAMPLE: Sample = Sample {
            max_threads: 0,
            num_threads: 0,
            region_uid: 0,
            runtime: 0.0,
            usertime: 0.0,
            energy: 0.0,
        };

        let mut elems = [SAMPLE; N];
        elems[0] = value;
        Self { elems, len: 1 }
    }
}
