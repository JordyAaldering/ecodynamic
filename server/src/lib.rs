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

    pub fn read(&mut self, region_uid: i32) -> Option<Demand> {
        self.letterbox.get_mut(&region_uid)
            .map(|controller| {
                controller.next_demand()
            })
    }

    pub fn update(&mut self, sample: Sample) -> Demand {
        let controller = self.letterbox.entry(sample.region_uid)
            .or_insert_with(|| (self.build)(&sample));
        controller.sample_received(sample);
        controller.next_demand()
    }
}
