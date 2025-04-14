use std::mem;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Sample {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// Total runtime of the previous iteration.
    pub runtime: f32,
    /// Total usertime of the previous iteration.
    pub usertime: f32,
    /// Total energy consumption of the previous iteration.
    pub energy: f32,
}

impl Sample {
    pub const SIZE: usize = mem::size_of::<Sample>();

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let [i0, i1, i2, i3] = self.region_uid.to_ne_bytes();
        let [r0, r1, r2, r3] = self.runtime.to_ne_bytes();
        let [u0, u1, u2, u3] = self.usertime.to_ne_bytes();
        let [e0, e1, e2, e3] = self.energy.to_ne_bytes();
        [i0, i1, i2, i3,
         r0, r1, r2, r3,
         u0, u1, u2, u3,
         e0, e1, e2, e3]
    }
}

impl From<[u8; Self::SIZE]> for Sample {
    fn from(buffer: [u8; Self::SIZE]) -> Self {
        let [i0, i1, i2, i3,
             r0, r1, r2, r3,
             u0, u1, u2, u3,
             e0, e1, e2, e3 ] = buffer;
        let region_uid = i32::from_ne_bytes([i0, i1, i2, i3]);
        let runtime = f32::from_ne_bytes([r0, r1, r2, r3]);
        let usertime = f32::from_ne_bytes([u0, u1, u2, u3]);
        let energy = f32::from_ne_bytes([e0, e1, e2, e3]);
        Self { region_uid, runtime, usertime, energy }
    }
}

pub(crate) struct SampleVec {
    samples: Vec<Sample>,
    size: usize,
}

impl SampleVec {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            samples: Vec::with_capacity(size),
            size,
        }
    }

    pub(crate) fn push(&mut self, sample: Sample) {
        debug_assert!(!self.is_full());
        self.samples.push(sample);
    }

    pub(crate) fn take(&mut self) -> Vec<Sample> {
        debug_assert!(self.is_full());
        let mut out = Vec::with_capacity(self.size);
        mem::swap(&mut self.samples, &mut out);
        out
    }

    pub(crate) fn is_full(&self) -> bool {
        self.samples.len() == self.size
    }
}
