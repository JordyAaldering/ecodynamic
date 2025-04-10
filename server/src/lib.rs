use std::collections::HashMap;

use controller::{Controller, Demand, Request, Sample};

pub const MTD_LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

pub struct Letterbox<Ctrl: Controller> {
    build: fn(Request) -> Ctrl,
    letterbox: HashMap<i32, Ctrl>,
}

impl<Ctrl: Controller> Letterbox<Ctrl> {
    pub fn new(build: fn(Request) -> Ctrl) -> Self {
        Self { build, letterbox: HashMap::new() }
    }

    pub fn read(&mut self, req: Request) -> Demand {
        self.letterbox.entry(req.region_uid)
            .or_insert_with(|| ((self.build)(req)))
            .next_demand()
    }

    pub fn update(&mut self, sample: Sample) {
        self.letterbox
            .get_mut(&sample.region_uid)
            .expect("Letterbox not initialized")
            .sample_received(sample);
    }
}
