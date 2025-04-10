use std::mem;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Request {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// The maximum number of threads allowed for this parallel region.
    pub max_threads: i32,
}

const SIZE: usize = mem::size_of::<Request>();

impl Request {
    pub fn to_bytes(self) -> [u8; SIZE] {
        let [i0, i1, i2, i3] = self.region_uid.to_ne_bytes();
        let [m0, m1, m2, m3] = self.max_threads.to_ne_bytes();
        [i0, i1, i2, i3,
         m0, m1, m2, m3]
    }
}

impl From<[u8; SIZE]> for Request {
    fn from(buffer: [u8; SIZE]) -> Self {
        let [i0, i1, i2, i3,
             m0, m1, m2, m3 ] = buffer;
        let region_uid = i32::from_ne_bytes([i0, i1, i2, i3]);
        let max_threads = i32::from_ne_bytes([m0, m1, m2, m3]);
        Self { region_uid, max_threads }
    }
}
