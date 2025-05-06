use std::collections::HashMap;

use controller::{control::Controller, message::*};

pub const MTD_LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

pub struct Letterbox<F>
    where F: Fn(Request) -> Box<dyn Controller>
{
    build: F,
    letterbox: HashMap<i32, Box<dyn Controller>>,
}

impl<F> Letterbox<F>
    where F: Fn(Request) -> Box<dyn Controller>
{
    pub fn new(build: F) -> Self {
        Self { build, letterbox: HashMap::new() }
    }

    pub fn try_get_demand(&mut self, req: Request) -> Demand {
        self.letterbox.entry(req.region_uid)
            .or_insert_with(|| (self.build)(req))
            .get_demand()
    }

    pub fn get_demand(&mut self, region_uid: i32) -> Demand {
        self.letterbox.get(&region_uid).unwrap()
            .get_demand()
    }

    pub fn update(&mut self, region_uid: i32, score: f32) {
        self.letterbox
            .get_mut(&region_uid)
            .expect("Letterbox not initialized")
            .sample_received(score);
    }
}
