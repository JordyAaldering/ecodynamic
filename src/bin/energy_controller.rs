use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use mtdynamic::{Bucket, Controller, SHM_LETTERBOX_NAME, SHM_SEMAPHORE_NAME};

fn ctrlc_handler() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);

        unsafe {
            let sem = libc::sem_open(SHM_SEMAPHORE_NAME, 0);
            libc::sem_post(sem);
        }
    }).unwrap();

    running
}

fn main() {
    println!("creating sem");
    let sem = unsafe { mtdynamic::init_semaphore() };
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

        // Signal received to update thread-count
        for bucket in &mut lb.buckets {
            match bucket {
                Bucket::Occupied(pid, fptr, ctrl, incoming, thread_count) => {
                    if incoming.len == 0 {
                        println!("pid {} fun ptr {:?} needs update", pid, fptr);
                        *thread_count = ctrl.adjust_threads(incoming.data.to_vec());
                        println!("Updated thread-count of {:?} to {}", fptr, *thread_count);
                    }
                },
                _ => { },
            }
        }
    }

    unsafe {
        println!("Unlinking semaphore");
        let res = libc::sem_unlink(SHM_SEMAPHORE_NAME);
        assert_eq!(res, 0);
        println!("Freeing shared memory");
        let res = libc::shm_unlink(SHM_LETTERBOX_NAME);
        assert_eq!(res, 0);
        println!("Closing shared");
        let res = libc::close(fd);
        assert_eq!(res, 0);
    };
}
