mod letterbox;

pub use letterbox::Letterbox;

use libc::{c_char, close, ftruncate, off_t, shm_open, shm_unlink, O_CREAT, O_RDWR, S_IRUSR, S_IWUSR};

pub const SHM_NAME: *const c_char = b"/mtdynamic\0".as_ptr() as *const c_char;

pub unsafe fn create_letterbox<'a>() -> (i32, &'a mut Letterbox) {
    let fd = shm_open(SHM_NAME, O_RDWR | O_CREAT, (S_IRUSR | S_IWUSR) as u32);
    assert!(fd >= 0);
    let res = ftruncate(fd, std::mem::size_of::<Letterbox>() as off_t);
    assert_eq!(res, 0);
    let lb = Letterbox::from_mmap(fd);
    (fd, lb)
}

pub unsafe fn free_shm(fd: i32) {
    let res = shm_unlink(SHM_NAME);
    assert_eq!(res, 0);
    let res = close(fd);
    assert_eq!(res, 0);
}
