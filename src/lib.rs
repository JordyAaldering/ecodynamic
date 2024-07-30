mod controller;
mod letterbox;

use std::{collections::HashMap, ffi::{c_char, CStr}};

use letterbox::Letterbox;
use controller::Controller;

pub struct MTDynamic {
    max_threads: i32,
    controllers: HashMap<String, (Controller, Letterbox)>,
}

#[no_mangle]
pub extern "C" fn MTDcreate(max_threads: i32, mtd_out: *mut *mut MTDynamic) {
    let mtd = MTDynamic {
        max_threads,
        controllers: HashMap::new(),
    };
    unsafe {
        *mtd_out = Box::into_raw(Box::new(mtd));
    }
}

#[no_mangle]
pub extern "C" fn MTDupdate(mtd: *mut &mut MTDynamic, funname: *const c_char, runtime_nanos: u64) {
    let mtd = unsafe { std::ptr::read(mtd) };

    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    if !mtd.controllers.contains_key(&funname) {
        let controller = Controller::new(mtd.max_threads);
        let letterbox = Letterbox::new(mtd.max_threads);
        mtd.controllers.insert(funname.clone(), (controller, letterbox));
    }

    let (ref mut controller, ref mut letterbox) = mtd.controllers.get_mut(&funname).unwrap();

    let len = letterbox.push(runtime_nanos);

    if len >= 20 {
        let num_threads = controller.adjust_threads(letterbox.take());
        println!("{} nr. threads from {} to {}", &funname, letterbox.num_threads, num_threads);
        letterbox.num_threads = num_threads;
    }
}

#[no_mangle]
pub extern "C" fn MTDgetNumThreads(mtd: *mut &mut MTDynamic, funname: *const c_char) -> i32 {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();
    if let Some((_, letterbox)) = mtd.controllers.get(&funname) {
        letterbox.num_threads
    } else {
        mtd.max_threads
    }
}
