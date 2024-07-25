pub struct Letterbox {
    pub runtimes: Vec<u64>,
    pub num_threads: i32,
}

impl Letterbox {
    pub fn new(max_threads: i32) -> Self {
        Letterbox {
            runtimes: Vec::new(),
            num_threads: max_threads,
        }
    }
}

#[no_mangle]
pub extern "C" fn MTDcreateLetterbox(num_threads: i32, letterbox_out: *mut *mut Letterbox) {
    let letterbox = Letterbox::new(num_threads);
    unsafe {
        *letterbox_out = Box::into_raw(Box::new(letterbox));
    }
}

#[no_mangle]
pub extern "C" fn MTDpushMetrics(letterbox: *mut &mut Letterbox, runtime_nanos: u64) {
    let letterbox = unsafe { std::ptr::read(letterbox) };
    letterbox.runtimes.push(runtime_nanos);
}

#[no_mangle]
pub extern "C" fn MTDgetNumThreads(letterbox: *mut &mut Letterbox) -> i32 {
    let letterbox = unsafe { std::ptr::read(letterbox) };
    letterbox.num_threads
}
