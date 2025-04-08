use std::mem;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Sample {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// The maximum number of threads allowed for this parallel region.
    pub max_threads: i32,
    /// The number of threads used by the previous iteration.
    pub num_threads: i32,
    /// Total runtime of the previous iteration.
    pub runtime: f32,
    /// Total usertime of the previous iteration.
    pub usertime: f32,
    /// Total energy consumption of the previous iteration.
    pub energy: f32,
}

const SIZE: usize = mem::size_of::<Sample>();

impl Sample {
    pub fn to_bytes(self) -> [u8; SIZE] {
        let [i0, i1, i2, i3] = self.region_uid.to_ne_bytes();
        let [m0, m1, m2, m3] = self.max_threads.to_ne_bytes();
        let [n0, n1, n2, n3] = self.num_threads.to_ne_bytes();
        let [r0, r1, r2, r3] = self.runtime.to_ne_bytes();
        let [u0, u1, u2, u3] = self.usertime.to_ne_bytes();
        let [e0, e1, e2, e3] = self.energy.to_ne_bytes();
        [i0, i1, i2, i3,
         m0, m1, m2, m3,
         n0, n1, n2, n3,
         r0, r1, r2, r3,
         u0, u1, u2, u3,
         e0, e1, e2, e3]
    }
}

impl From<[u8; SIZE]> for Sample {
    fn from(buffer: [u8; SIZE]) -> Self {
        let [i0, i1, i2, i3,
             m0, m1, m2, m3,
             n0, n1, n2, n3,
             r0, r1, r2, r3,
             u0, u1, u2, u3,
             e0, e1, e2, e3 ] = buffer;
        let region_uid = i32::from_ne_bytes([i0, i1, i2, i3]);
        let max_threads = i32::from_ne_bytes([m0, m1, m2, m3]);
        let num_threads = i32::from_ne_bytes([n0, n1, n2, n3]);
        let runtime = f32::from_ne_bytes([r0, r1, r2, r3]);
        let usertime = f32::from_ne_bytes([u0, u1, u2, u3]);
        let energy = f32::from_ne_bytes([e0, e1, e2, e3]);
        Self { max_threads, num_threads, region_uid, runtime, usertime, energy }
    }
}
