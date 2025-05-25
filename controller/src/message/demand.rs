use std::mem;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Demand {
    /// Recommended number of threads to use for the next parallel iteration.
    pub num_threads: i32,
    /// Maximum allowed CPU power in micro watts.
    pub power_limit_uw: u64,
}

impl Demand {
    pub const SIZE: usize = mem::size_of::<i32>();

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        self.num_threads.to_ne_bytes()
    }
}

impl From<[u8; Self::SIZE]> for Demand {
    fn from(buffer: [u8; Self::SIZE]) -> Self {
        let num_threads = i32::from_ne_bytes(buffer);
        Self { num_threads, power_limit_uw: 0 }
    }
}
