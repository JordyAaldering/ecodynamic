mod controller;
pub mod controller_runtime;
mod letterbox;
mod libc;

use std::collections::BTreeMap;

pub use letterbox::{Letterbox, Sample};
use controller::Controller;

pub struct MTDynamic {
    max_threads: i32,
    num_measurements_per_adjustment: usize,
    controllers: BTreeMap<String, (Controller, Letterbox)>,
}

impl MTDynamic {
    pub fn new(max_threads: i32, num_measurements_per_adjustment: usize) -> Self {
        MTDynamic {
            max_threads,
            num_measurements_per_adjustment,
            controllers: BTreeMap::new(),
        }
    }

    pub fn update<S: AsRef<str>>(&mut self, funname: S, runtime: f64, usertime: f64, energy: f64) {
        if !self.controllers.contains_key(funname.as_ref()) {
            let controller = Controller::new(self.max_threads);
            let letterbox = Letterbox::new(self.max_threads, self.num_measurements_per_adjustment);
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
            letterbox.num_threads
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
