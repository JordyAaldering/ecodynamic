use super::SHM_NAME;
use std::hash::{DefaultHasher, Hash, Hasher};

/// A letterbox is a hashmap-like mapping from unique identifiers (function
/// pointers) to incoming (runtime/energy measurements) and outgoing
/// (thread-count) data.
#[repr(C)]
pub struct Letterbox {
    len: usize,
    buckets: [Bucket; 32],
}

#[repr(C)]
enum Bucket {
    Empty,
    Occupied(u64, Incoming, Outgoing),
    Tombstone,
}

type Incoming = [f32; 20];
type Outgoing = i32;

impl Letterbox {
    pub fn get(&self, key: u64) -> Option<(&Incoming, &Outgoing)> {
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

    pub fn get_mut(&mut self, key: u64) -> Option<(&Incoming, &Outgoing)> {
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

    fn get_hash(&self, key: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
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
}

#[no_mangle]
unsafe extern "C" fn open_letterbox() -> *mut Letterbox {
    use libc::{shm_open, O_RDWR, S_IRUSR, S_IWUSR};
    let fd = shm_open(SHM_NAME, O_RDWR, (S_IRUSR | S_IWUSR) as u32);
    if fd < 0 {
        eprintln!("resource controller is not running");
        std::ptr::null_mut()
    } else {
        Letterbox::from_mmap(fd)
    }
}
