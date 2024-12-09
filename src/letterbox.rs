use libc::uintptr_t;

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{EnergyController, SHM_NAME};

/// A letterbox is a hashmap-like mapping from unique identifiers (function
/// pointers) to incoming (runtime/energy measurements) and outgoing
/// (thread-count) data.
#[repr(C)]
pub struct Letterbox {
    len: usize,
    pub buckets: [Bucket; 32],
}

#[repr(C)]
pub enum Bucket {
    Empty,
    Occupied(uintptr_t, Incoming, Outgoing),
    Tombstone,
}

#[repr(C)]
pub struct Incoming {
    pub len: usize,
    pub data: [f32; 20],
}

#[repr(transparent)]
pub struct Outgoing {
    pub controller: EnergyController,
}

#[no_mangle]
unsafe extern "C" fn MTD_letterbox_open() -> *mut Letterbox {
    use libc::{shm_open, O_RDWR, S_IRUSR, S_IWUSR};
    println!("opening letterbox");
    let fd = shm_open(SHM_NAME, O_RDWR, (S_IRUSR | S_IWUSR) as u32);
    if fd < 0 {
        eprintln!("resource controller is not running");
        std::ptr::null_mut()
    } else {
        Letterbox::from_mmap(fd)
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_letterbox_push(lb: &mut Letterbox, key: uintptr_t, value: f32) -> usize {
    println!("push {:?} = {}", key, value);
    if let Some((incoming, _)) = lb.get_mut(key) {
        incoming.data[incoming.len] = value;
        let res = incoming.len + 1;
        incoming.len = res % 20;
        res
    } else {
        lb.insert(key, value);
        1
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_thread_count(lb: &mut Letterbox, key: uintptr_t) -> u32 {
    if let Some((_, controller)) = lb.get(key) {
        controller.controller.num_threads.round() as u32
    } else {
        16
    }
}

impl Letterbox {
    pub unsafe fn from_mmap<'a>(fd: i32) -> &'a mut Self {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            std::mem::size_of::<Self>(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0
        );
        &mut *(ptr as *mut Self)
    }

    pub fn insert(&mut self, key: uintptr_t, value: f32) {
        let start_idx = self.get_hash(key);

        let (lhs, rhs) = self.buckets.split_at_mut(start_idx);

        for bucket in rhs.iter_mut().chain(lhs.iter_mut()) {
            match bucket {
                Bucket::Empty |
                Bucket::Tombstone => {
                    let mut data = [0.0; 20];
                    data[0] = value;
                    *bucket = Bucket::Occupied(
                        key,
                        Incoming { len: 1, data },
                        Outgoing { controller: EnergyController::new(16) }
                    );
                    break;
                }
                Bucket::Occupied(_, _, _) => { },
            }
        }
    }

    pub fn get(&self, key: uintptr_t) -> Option<(&Incoming, &Outgoing)> {
        let start_idx = self.get_hash(key);

        let (lhs, rhs) = self.buckets.split_at(start_idx);

        for bucket in rhs.iter().chain(lhs.iter()) {
            match bucket {
                Bucket::Empty => return None,
                Bucket::Occupied(k, i, o) if key == *k => return Some((i, o)),
                _ => { },
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: uintptr_t) -> Option<(&mut Incoming, &mut Outgoing)> {
        let start_idx = self.get_hash(key);

        let (lhs, rhs) = self.buckets.split_at_mut(start_idx);

        for bucket in rhs.iter_mut().chain(lhs.iter_mut()) {
            match bucket {
                Bucket::Empty => return None,
                Bucket::Occupied(k, i, o) if key == *k => return Some((i, o)),
                _ => { },
            }
        }

        None
    }

    fn get_hash(&self, key: uintptr_t) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }
}
