use std::collections::HashMap;

use crate::{message::*, Controller};

pub struct Letterbox<F>
    where F: Fn(Request) -> Box<dyn Controller>
{
    build: F,
    letterbox: HashMap<i32, (Vec<Sample>, Box<dyn Controller>)>,
}

impl<F> Letterbox<F>
    where F: Fn(Request) -> Box<dyn Controller>
{
    pub fn new(build: F) -> Self {
        Self { build, letterbox: HashMap::new() }
    }

    pub fn get_or_insert(&mut self, req: Request) -> &mut (Vec<Sample>, Box<dyn Controller>) {
        self.letterbox.entry(req.region_uid)
            .or_insert_with(|| (Vec::new(), (self.build)(req)))
    }

    pub fn get(&mut self, region_uid: i32) -> &mut (Vec<Sample>, Box<dyn Controller>) {
        self.letterbox.get_mut(&region_uid).unwrap()
    }
}
