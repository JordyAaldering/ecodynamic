use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use mtdynamic::shm;

fn ctrlc_handler() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    }).unwrap();

    running
}

fn main() {
    let (fd, lb) = unsafe { shm::create_letterbox() };

    let running = ctrlc_handler();
    while running.load(Ordering::Relaxed) {

        todo!()

    }

    unsafe { shm::free_shm(fd) };
}
