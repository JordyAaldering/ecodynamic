use std::mem;

/// System-wide demands that have to be set by this controller.
#[derive(Clone, Debug, Default)]
pub struct GlobalDemand {
    /// Maximum allowed CPU power as a fraction beteen (0,1].
    pub power_limit_pct: f32,
}

/// Application-specific demands that have to be set by the controlled application.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct LocalDemand {
    /// Recommended number of threads to use for the next parallel iteration.
    pub threads_pct: f32,
}

impl LocalDemand {
    pub const SIZE: usize = mem::size_of::<Self>();

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        self.threads_pct.to_ne_bytes()
    }
}

impl From<[u8; Self::SIZE]> for LocalDemand {
    fn from(buffer: [u8; Self::SIZE]) -> Self {
        let threads_pct = f32::from_ne_bytes(buffer);
        Self { threads_pct }
    }
}
