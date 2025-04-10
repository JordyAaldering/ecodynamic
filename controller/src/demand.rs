use std::mem;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Demand {
    /// Recommended number of threads to use for the next parallel iteration.
    pub num_threads: i32,
}

pub const SIZE: usize = mem::size_of::<Demand>();

impl Demand {
    pub fn to_bytes(self) -> [u8; SIZE] {
        self.num_threads.to_ne_bytes()
    }
}

impl From<[u8; SIZE]> for Demand {
    fn from(buffer: [u8; SIZE]) -> Self {
        let num_threads = i32::from_ne_bytes(buffer);
        Self { num_threads }
    }
}
