include!(concat!(env!("OUT_DIR"), "/config.rs"));

pub mod controller;
pub mod letterbox;

pub use controller::*;
pub use letterbox::*;

use libc::{c_char, getpid, sem_post, shm_open, uintptr_t, O_RDWR, S_IRUSR, S_IWUSR};

pub const SHM_LETTERBOX_NAME: *const c_char = b"/mtd_shm_letterbox\0".as_ptr() as *const c_char;
pub const SHM_SEMAPHORE_NAME: *const c_char = b"/mtd_shm_semaphore\0".as_ptr() as *const c_char;

#[no_mangle]
unsafe extern "C" fn MTD_letterbox_open() -> *mut Letterbox {
    let fd = shm_open(SHM_LETTERBOX_NAME, O_RDWR, (S_IRUSR | S_IWUSR) as u32);
    if fd < 0 {
        std::ptr::null_mut()
    } else {
        Letterbox::from_mmap(fd)
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_letterbox_push(lb: &mut Letterbox, key: uintptr_t, value: f32) {
    let pid = unsafe { getpid() };

    println!("push {:?} = {}", key, value);
    if let Some(incoming) = lb.get_incoming_mut(key) {
        assert!(incoming.len < 20);
        incoming.data[incoming.len] = value;
        incoming.len += 1;

        if incoming.len == 20 {
            let sem = libc::sem_open(SHM_SEMAPHORE_NAME, 0);
            assert_ne!(sem, std::ptr::null_mut());
            let res = sem_post(sem);
            assert_eq!(res, 0);
            incoming.len = 0;
        }
    } else {
        println!("pushing new fptr {}", key);
        assert!(lb.len < lb.buckets.len());
        lb.insert(pid, key, value);
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_thread_count(lb: &mut Letterbox, key: uintptr_t) -> i32 {
    if let Some(thread_count) = lb.get_thread_count(key) {
        *thread_count
    } else {
        16
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_free_buckets(lb: &mut Letterbox) {
    let self_pid = unsafe { getpid() };
    println!("Freeing letterboxes of {}", self_pid);

    for bucket in lb.buckets.iter_mut() {
        match bucket {
            BucketType::Occupied(Bucket { pid, fptr, .. }) if self_pid == *pid => {
                println!("Cleaning {}:{}", self_pid, fptr);
                *bucket = BucketType::Tombstone;
                lb.len -= 1;
            }
            _ => { },
        }
    }
}
