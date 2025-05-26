use std::mem;

/// System-wide demands that have to be set by this controller.
#[derive(Clone, Debug, Default)]
pub struct GlobalDemand {
    /// Maximum allowed CPU power in micro watts.
    pub power_limit_uw: u64,
}

/// Application-specific demands that have to be set by the controlled application.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct LocalDemand {
    /// Recommended number of threads to use for the next parallel iteration.
    pub num_threads: i32,
}

impl LocalDemand {
    pub const SIZE: usize = mem::size_of::<Self>();

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        self.num_threads.to_ne_bytes()
    }
}

impl From<[u8; Self::SIZE]> for LocalDemand {
    fn from(buffer: [u8; Self::SIZE]) -> Self {
        let num_threads = i32::from_ne_bytes(buffer);
        Self { num_threads }
    }
}
