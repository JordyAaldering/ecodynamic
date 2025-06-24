use std::mem;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Request {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// The same region might have a varying input size.
    pub problem_size: i32,
}

impl Request {
    pub const SIZE: usize = mem::size_of::<Request>();

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let [i0, i1, i2, i3] = self.region_uid.to_ne_bytes();
        let [s0, s1, s2, s3] = self.problem_size.to_ne_bytes();
        [i0, i1, i2, i3,
         s0, s1, s2, s3]
    }
}

impl From<[u8; Self::SIZE]> for Request {
    fn from(buffer: [u8; Self::SIZE]) -> Self {
        let [i0, i1, i2, i3,
             s0, s1, s2, s3 ] = buffer;
        let region_uid = i32::from_ne_bytes([i0, i1, i2, i3]);
        let problem_size = i32::from_ne_bytes([s0, s1, s2, s3]);
        Self { region_uid, problem_size }
    }
}
