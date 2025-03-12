use std::collections::HashMap;

use controller::corridor_controller::Controller;
//use controller::delta_controller::Controller;

use crate::{Demand, Sample};

#[derive(Default)]
pub struct Letterbox<const N: usize> {
    letterbox: HashMap<i32, (Controller, Samples<N>)>,
}

impl<const N: usize> Letterbox<N> {
    pub fn update(&mut self, msg: Sample) -> Demand {
        let threads = if let Some((controller, samples)) = self.letterbox.get_mut(&msg.uid) {
            samples.push(msg.val);

            if samples.len >= N {
                controller.adjust_threads(samples.take())
            } else {
                controller.threads()
            }
        } else {
            let controller = Controller::new(msg.max);
            let samples = Samples::from(msg.val);
            self.letterbox.insert(msg.uid, (controller, samples));
            msg.max
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
