use libc::{pid_t, uintptr_t};

use std::{hash::{DefaultHasher, Hash, Hasher}, mem, ptr};

use crate::{controller::Controller, NUM_LETTERBOXES, NUM_SAMPLES};

#[repr(C)]
pub struct Letterbox {
    pub len: usize,
    pub buckets: [BucketType; NUM_LETTERBOXES],
}

#[repr(C)]
pub enum BucketType {
    Empty,
    Tombstone,
    Occupied(Bucket),
}

#[repr(C)]
pub struct Bucket {
    pub pid: pid_t,
    pub fptr: uintptr_t,
    pub ctrl: Controller,
    pub samples: Incoming,
    pub thread_count: i32,
}

#[repr(C)]
pub struct Incoming {
    pub len: usize,
    pub data: [f32; NUM_SAMPLES],
}

impl Letterbox {
    pub unsafe fn from_mmap<'a>(fd: i32) -> &'a mut Self {
        let size = mem::size_of::<Self>();
        println!("Creating mmap of {}b", size);
        let ptr = libc::mmap(ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
        assert_ne!(ptr, ptr::null_mut());
        println!("Casting mmap to Letterbox");
        &mut *(ptr as *mut Self)
    }

    pub fn insert(&mut self, pid: pid_t, key: uintptr_t, value: f32) {
        assert!(self.len < self.buckets.len());
        let start_idx = self.get_hash(key);
        for bucket in self.iter_mut_from(start_idx) {
            match bucket {
                BucketType::Empty |
                BucketType::Tombstone => {
                    println!("found a spot for {}", key);
                    let mut data = [0.0; NUM_SAMPLES];
                    data[0] = value;
                    *bucket = BucketType::Occupied(Bucket {
                        pid: pid,
                        fptr: key,
                        ctrl: Controller::new(16),
                        samples: Incoming { len: 1, data },
                        thread_count: 16
                    });
                    println!("inserted {}", key);
                    return;
                }
                _ => { },
            }
        }
    }

    pub fn get_thread_count(&self, key: uintptr_t) -> Option<&i32> {
        let start_idx = self.get_hash(key);
        for bucket in self.iter_from(start_idx) {
            match bucket {
                BucketType::Empty => return None,
                BucketType::Occupied (Bucket { fptr, thread_count, .. }) if key == *fptr
                    => return Some(thread_count),
                _ => { /* tombstone or not a match; keep going */ },
            }
        }

        None
    }

    pub fn get_incoming_mut(&mut self, key: uintptr_t) -> Option<&mut Incoming> {
        let start_idx = self.get_hash(key);
        for bucket in self.iter_mut_from(start_idx) {
            match bucket {
                BucketType::Empty => return None,
                BucketType::Occupied(Bucket { fptr, samples, .. }) if key == *fptr
                    => return Some(samples),
                _ => { },
            }
        }

        None
    }

    fn iter_from(&self, start_idx: usize) -> impl Iterator<Item = &BucketType> {
        let (lhs, rhs) = self.buckets.split_at(start_idx);
        rhs.iter().chain(lhs.iter())
    }

    fn iter_mut_from(&mut self, start_idx: usize) -> impl Iterator<Item = &mut BucketType> {
        let (lhs, rhs) = self.buckets.split_at_mut(start_idx);
        rhs.iter_mut().chain(lhs.iter_mut())
    }

    fn get_hash(&self, key: uintptr_t) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }
}
