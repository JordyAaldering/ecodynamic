mod controller;
mod letterbox;

use letterbox::Letterbox;
use controller::Controller;

#[no_mangle]
pub extern "C" fn MTDcreateLetterbox(num_threads: i32, letterbox_out: *mut *mut Letterbox) {
    let letterbox = Letterbox::new(num_threads);
    unsafe {
        *letterbox_out = Box::into_raw(Box::new(letterbox));
    }
}

#[no_mangle]
pub extern "C" fn MTDcreateController(num_threads: i32, controller_out: *mut *mut Controller) {
    let controller = Controller::new(num_threads);
    unsafe {
        *controller_out = Box::into_raw(Box::new(controller));
    }
}

#[no_mangle]
pub extern "C" fn MTDupdateController(controller: *mut &mut Controller, letterbox: *mut &mut crate::Letterbox, runtime_nanos: u64) {
    if runtime_nanos == 0 {
        return;
    }

    let letterbox = unsafe { std::ptr::read(letterbox) };

    letterbox.runtimes.push(runtime_nanos);

    if letterbox.runtimes.len() >= 20 {
        let controller = unsafe { std::ptr::read(controller) };

        let num_threads = controller.adjust_threads(&letterbox.runtimes);
        println!("Controller num threads from {} to {}", letterbox.num_threads, num_threads);
        letterbox.num_threads = num_threads;

        letterbox.runtimes.clear();
    }
}

#[no_mangle]
pub extern "C" fn MTDgetNumThreads(letterbox: *mut &mut Letterbox) -> i32 {
    let letterbox = unsafe { std::ptr::read(letterbox) };
    letterbox.num_threads
}
