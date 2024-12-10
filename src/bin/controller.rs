use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use libc::{ftruncate, off_t, sem_t, shm_open, O_CREAT, O_EXCL, O_RDWR, S_IRUSR, S_IWUSR};
use mtdynamic::{Bucket, BucketType, Controller, Letterbox, SHM_LETTERBOX_NAME, SHM_SEMAPHORE_NAME};

unsafe fn init_semaphore() -> *mut sem_t {
    let sem = libc::sem_open(SHM_SEMAPHORE_NAME, O_RDWR | O_CREAT | O_EXCL, (S_IRUSR | S_IWUSR) as u32, 0);
    assert_ne!(sem, std::ptr::null_mut());
    sem
}

unsafe fn init_letterbox<'a>() -> (i32, &'a mut Letterbox) {
    let fd = shm_open(SHM_LETTERBOX_NAME, O_RDWR | O_CREAT | O_EXCL, (S_IRUSR | S_IWUSR) as u32);
    println!("shm_open returned {}", fd);
    assert!(fd >= 0);
    let res = ftruncate(fd, std::mem::size_of::<Letterbox>() as off_t);
    println!("ftruncate returned {}", res);
    assert_eq!(res, 0);
    let lb = Letterbox::from_mmap(fd);
    (fd, lb)
}

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
    let sem = unsafe { init_semaphore() };
    let (fd, lb) = unsafe { init_letterbox() };

    let running = ctrlc_handler();
    while running.load(Ordering::Relaxed) {
        // Wait for a signal to recompute thread-count
        unsafe { libc::sem_wait(sem) };

        if !running.load(Ordering::Relaxed) {
            // ctrlc_handler posted this semaphore
            break
        }

        // Signal received to update thread-count
        for bucket in &mut lb.buckets {
            match bucket {
                BucketType::Occupied(Bucket { pid, fptr, ctrl, samples, thread_count }) => {
                    if samples.len == 0 {
                        *thread_count = ctrl.adjust_threads(samples.data.to_vec());
                        println!("Changed thread-count of {}:{:?} to {}", pid, fptr, *thread_count);
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
        println!("Closing shared memory");
        let res = libc::close(fd);
        assert_eq!(res, 0);
    };
}
