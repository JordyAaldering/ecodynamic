mod controller;
mod letterbox;
mod util;

use std::{collections::HashMap, ffi::{c_char, CStr}};

use letterbox::Letterbox;
use controller::Controller;

pub struct MTDynamic {
    max_threads: i32,
    num_measurements_per_adjustment: usize,
    controllers: HashMap<String, (Controller, Letterbox)>,
}

#[no_mangle]
pub extern "C" fn MTDcreate(max_threads: i32, num_measurements_per_adjustment: usize, mtd_out: *mut *mut MTDynamic) {
    let mtd = MTDynamic {
        max_threads,
        num_measurements_per_adjustment,
        controllers: HashMap::new(),
    };
    unsafe {
        *mtd_out = Box::into_raw(Box::new(mtd));
    }
}

#[no_mangle]
pub extern "C" fn MTDupdate(mtd: *mut &mut MTDynamic, funname: *const c_char, runtime_ns: u64, energy_uj: u64) {
    if energy_uj == 0 {
        return;
    }

    let mtd = unsafe { std::ptr::read(mtd) };

    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    if !mtd.controllers.contains_key(&funname) {
        let controller = Controller::new(mtd.max_threads);
        let letterbox = Letterbox::new(mtd.max_threads);
        mtd.controllers.insert(funname.clone(), (controller, letterbox));
    }

    let (ref mut controller, ref mut letterbox) = mtd.controllers.get_mut(&funname).unwrap();

    let num_measurements = letterbox.push(runtime_ns, energy_uj);
    if num_measurements >= mtd.num_measurements_per_adjustment {
        let num_threads = controller.adjust_threads(letterbox.take());
        println!("{} nr. threads from {} to {}", &funname, letterbox.num_threads, num_threads);
        letterbox.num_threads = num_threads;
    }
}

#[no_mangle]
pub extern "C" fn MTDnumThreads(mtd: *mut &mut MTDynamic, funname: *const c_char) -> i32 {
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    let mtd = unsafe { std::ptr::read(mtd) };
    if let Some((_, letterbox)) = mtd.controllers.get(&funname) {
        letterbox.num_threads
    } else {
        mtd.max_threads
    }
}

#[no_mangle]
pub extern "C" fn MTDfree(mtd: *mut MTDynamic) {
    let mtd = unsafe { std::ptr::read(mtd) };
    for (name, (_, letterbox)) in &mtd.controllers {
        println!("{}: {:?}", name, letterbox.samples);
    }
    drop(mtd);
}
