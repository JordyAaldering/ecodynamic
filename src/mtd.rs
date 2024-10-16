use std::collections::BTreeMap;

use crate::controller::Controller;
use crate::controller_runtime::ControllerRuntime;
use crate::letterbox::{Letterbox, Sample};
use crate::controller_energy::ControllerEnergy;

pub struct MtdBuilder {
    max_threads: i32,
    controller_type: ControllerType,
    num_measurements_per_adjustment: usize,
}

pub enum ControllerType {
    Energy,
    Runtime,
}

impl MtdBuilder {
    pub fn new(max_threads: i32) -> Self {
        Self {
            max_threads,
            num_measurements_per_adjustment: 10,
            controller_type: ControllerType::Energy,
        }
    }

    pub fn energy(mut self) -> Self {
        self.controller_type = ControllerType::Energy;
        self
    }

    pub fn runtime(mut self) -> Self {
        self.controller_type = ControllerType::Runtime;
        self
    }

    pub fn letterbox_size(mut self, num_measurements_per_adjustment: usize) -> Self {
        self.num_measurements_per_adjustment = num_measurements_per_adjustment;
        self
    }

    pub fn build(self) -> MTDynamic {
        let controller_fn: Box<dyn Fn() -> Box<dyn Controller>> = match self.controller_type {
            ControllerType::Energy => Box::new(move || Box::new(ControllerEnergy::new(self.max_threads))),
            ControllerType::Runtime => Box::new(move || Box::new(ControllerRuntime::new(self.max_threads))),
        };

        MTDynamic {
            max_threads: self.max_threads,
            num_measurements_per_adjustment: self.num_measurements_per_adjustment,
            controller_fn: controller_fn,
            controllers: BTreeMap::new(),
        }
    }
}

pub struct MTDynamic {
    max_threads: i32,
    num_measurements_per_adjustment: usize,
    controller_fn: Box<dyn Fn() -> Box<dyn Controller>>,
    pub controllers: BTreeMap<String, (Box<dyn Controller>, Letterbox)>,
}

impl MTDynamic {
    pub fn update<S: AsRef<str>>(&mut self, funname: S, runtime: f64, usertime: f64, energy: f64) {
        if !self.controllers.contains_key(funname.as_ref()) {
            let controller = (self.controller_fn)();
            let letterbox = Letterbox::new(self.max_threads as f64, self.num_measurements_per_adjustment);
            self.controllers.insert(funname.as_ref().to_string(), (controller, letterbox));
        }

        let (ref mut controller, ref mut letterbox) = self.controllers.get_mut(funname.as_ref()).unwrap();

        let num_measurements = letterbox.push(Sample::new(runtime, usertime, energy));
        if num_measurements >= self.num_measurements_per_adjustment {
            let samples = letterbox.take();
            let num_threads = controller.adjust_threads(samples);
            //println!("{} nr. threads from {} to {}", funname.as_ref(), letterbox.num_threads, num_threads);
            letterbox.num_threads = num_threads;
        }
    }

    pub fn num_threads<S: AsRef<str>>(&self, funname: S) -> i32 {
        if let Some((_, letterbox)) = self.controllers.get(funname.as_ref()) {
            letterbox.num_threads.round() as i32
        } else {
            self.max_threads
        }
    }
}

impl std::fmt::Debug for MTDynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, (_, letterbox)) in &self.controllers {
            if letterbox.history.len() > 1 {
                f.write_fmt(format_args!("{},{:?},", name, letterbox))?;
            }
        }
        Ok(())
    }
}
