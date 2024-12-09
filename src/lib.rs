mod controller;
mod letterbox;

pub use controller::*;
pub use letterbox::*;

use libc::{c_char, ftruncate, off_t, sem_t, shm_open, O_CREAT, O_EXCL, O_RDWR, S_IRUSR, S_IWUSR};

pub const SHM_LETTERBOX_NAME: *const c_char = b"/mtd_shm_letterbox\0".as_ptr() as *const c_char;
pub const SHM_SEMAPHORE_NAME: *const c_char = b"/mtd_shm_semaphore\0".as_ptr() as *const c_char;

pub unsafe fn init_semaphore() -> *mut sem_t {
    let sem = libc::sem_open(SHM_SEMAPHORE_NAME, O_RDWR | O_CREAT | O_EXCL, (S_IRUSR | S_IWUSR) as u32, 0);
    assert_ne!(sem, std::ptr::null_mut());
    sem
}

pub unsafe fn init_letterbox<'a>() -> (i32, &'a mut Letterbox) {
    let fd = shm_open(SHM_LETTERBOX_NAME, O_RDWR | O_CREAT | O_EXCL, (S_IRUSR | S_IWUSR) as u32);
    println!("shm_open returned {}", fd);
    assert!(fd >= 0);
    let res = ftruncate(fd, std::mem::size_of::<Letterbox>() as off_t);
    println!("ftruncate returned {}", res);
    assert_eq!(res, 0);
    let lb = Letterbox::from_mmap(fd);
    (fd, lb)
}
