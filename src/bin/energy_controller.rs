use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use mtdynamic::{Bucket, SEM_NAME};

fn ctrlc_handler() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);

        unsafe {
            let sem = libc::sem_open(SEM_NAME, 0);
            libc::sem_post(sem);
        }
    }).unwrap();

    running
}

fn main() {
    unsafe {
        libc::shm_unlink(mtdynamic::SHM_NAME)
    };

    println!("creating sem");
    let sem = unsafe {
        mtdynamic::init_semaphore()
    };

    let (fd, lb) = unsafe { mtdynamic::init_letterbox() };
    println!("Letterbox created");

    let running = ctrlc_handler();
    while running.load(Ordering::Relaxed) {
        // Wait for a signal to recompute thread-count
        println!("waiting");
        unsafe { libc::sem_wait(sem) };

        if !running.load(Ordering::Relaxed) {
            // ctrlc_handler posted this semaphore
            break
        }

        println!("got signal, woo!");

        // Signal received to update thread-count
        for bucket in &mut lb.buckets {
            match bucket {
                Bucket::Occupied(pid, fptr, incoming, outgoing) => {
                    if incoming.len == 0 {
                        println!("pid {} fun ptr {:?} needs update", pid, fptr);
                        outgoing.controller.adjust_threads(incoming.data.to_vec());
                        println!("Updated thread-count of {:?} to {}", fptr, outgoing.controller.num_threads);
                    }
                },
                _ => { },
            }
        }
    }

    unsafe {
        println!("Unlinking semaphore");
        libc::sem_unlink(SEM_NAME);
        println!("Freeing shared memory");
        mtdynamic::free_shm(fd);
    };
}
