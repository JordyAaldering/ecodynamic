use std::collections::HashMap;

use controller::*;

use crate::SampleVec;

pub struct Letterbox<BuildFn>
    where BuildFn: Fn(Request) -> Box<dyn Controller>
{
    pub build_fn: BuildFn,
    pub letterbox: HashMap<i32, (SampleVec, Box<dyn Controller>)>,
}

impl<BuildFn> Letterbox<BuildFn>
    where BuildFn: Fn(Request) -> Box<dyn Controller>
{
    pub fn new(build: BuildFn) -> Self {
        Self { build_fn: build, letterbox: HashMap::new() }
    }
}
