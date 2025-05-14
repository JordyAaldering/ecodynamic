use std::collections::HashMap;

use controller::*;

pub struct Letterbox<BuildFn>
    where BuildFn: Fn(Request) -> Box<dyn Controller>
{
    pub build_fn: BuildFn,
    pub letterbox: HashMap<i32, (Vec<Sample>, Box<dyn Controller>)>,
}

impl<BuildFn> Letterbox<BuildFn>
    where BuildFn: Fn(Request) -> Box<dyn Controller>
{
    pub fn new(build_fn: BuildFn) -> Self {
        Self { build_fn, letterbox: HashMap::new() }
    }
}
