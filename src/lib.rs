mod clamp;
mod controller;
mod letterbox;

use std::{collections::HashMap, ffi::{c_char, CStr}};

use letterbox::{Letterbox, Sample};
use controller::Controller;

pub struct MTDynamic {
    max_threads: i32,
    num_measurements_per_adjustment: usize,
    controllers: HashMap<String, (Controller, Letterbox)>,
}

impl MTDynamic {
    pub fn new(max_threads: i32, num_measurements_per_adjustment: usize) -> Self {
        MTDynamic {
            max_threads,
            num_measurements_per_adjustment,
            controllers: HashMap::new(),
        }
    }

    pub fn update<S: AsRef<str>>(&mut self, funname: S, realtime_ns: u64, usertime_ns: u64, energy_uj: u64) {
        if !self.controllers.contains_key(funname.as_ref()) {
            let controller = Controller::new(self.max_threads);
            let letterbox = Letterbox::new(self.max_threads, self.num_measurements_per_adjustment);
            self.controllers.insert(funname.as_ref().to_string(), (controller, letterbox));
        }

        let (ref mut controller, ref mut letterbox) = self.controllers.get_mut(funname.as_ref()).unwrap();

        let num_measurements = letterbox.push(Sample::new(realtime_ns, usertime_ns, energy_uj));
        if num_measurements >= self.num_measurements_per_adjustment {
            let samples = letterbox.take();
            let num_threads = controller.adjust_threads(samples);
            println!("{} nr. threads from {} to {}", funname.as_ref(), letterbox.num_threads(), num_threads);
            letterbox.update_threads(num_threads);
        }
    }

    pub fn num_threads<S: AsRef<str>>(&self, funname: S) -> i32 {
        if let Some((_, letterbox)) = self.controllers.get(funname.as_ref()) {
            letterbox.num_threads()
        } else {
            self.max_threads
        }
    }
}

impl std::fmt::Debug for MTDynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, (_, letterbox)) in &self.controllers {
            f.write_fmt(format_args!("{}: {:?}", name, letterbox))?;
        }
        Ok(())
    }
}

#[no_mangle]
extern "C" fn MTDcreate(max_threads: i32, num_measurements_per_adjustment: usize, mtd_out: *mut *mut MTDynamic) {
    let mtd = MTDynamic::new(max_threads, num_measurements_per_adjustment);
    unsafe {
        *mtd_out = Box::into_raw(Box::new(mtd));
    }
}

#[no_mangle]
extern "C" fn MTDupdate(mtd: *mut &mut MTDynamic, funname: *const c_char, realtime_ns: u64, usertime_ns: u64, energy_uj: u64) {
    if energy_uj == 0 {
        return;
    }

    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();
    mtd.update(funname, realtime_ns, usertime_ns, energy_uj);
}

#[no_mangle]
extern "C" fn MTDnumThreads(mtd: *mut &mut MTDynamic, funname: *const c_char) -> i32 {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();
    mtd.num_threads(funname)
}

#[no_mangle]
extern "C" fn MTDfree(mtd: *mut MTDynamic) {
    let mtd = unsafe { std::ptr::read(mtd) };
    println!("{:?}", mtd);
    drop(mtd);
}
