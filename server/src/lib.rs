use std::collections::HashMap;

use controller::{Controller, Demand, Sample};

pub const MTD_LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

pub struct Letterbox<Ctrl: Controller> {
    build: fn(&Sample) -> Ctrl,
    letterbox: HashMap<i32, Ctrl>,
}

impl<Ctrl: Controller> Letterbox<Ctrl> {
    pub fn new(build: fn(&Sample) -> Ctrl) -> Self {
        Self { build, letterbox: HashMap::new() }
    }

    pub fn update(&mut self, sample: Sample) -> Demand {
        let controller = self.letterbox.entry(sample.region_uid)
            .or_insert_with(|| (self.build)(&sample));
        controller.sample_received(sample);
        controller.next_demand()
    }
}
