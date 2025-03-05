#[repr(C)]
#[derive(Debug)]
pub struct Incoming {
    pub pid: i32,
    pub fid: i32,
    pub val: f32,
}

impl Incoming {
    pub fn to_bytes(self) -> [u8; 12] {
        let [p0, p1, p2, p3] = self.pid.to_ne_bytes();
        let [f0, f1, f2, f3] = self.fid.to_ne_bytes();
        let [v0, v1, v2, v3] = self.val.to_ne_bytes();
        [p0, p1, p2, p3,
         f0, f1, f2, f3,
         v0, v1, v2, v3]
    }
}

impl From<[u8; 12]> for Incoming {
    fn from(buffer: [u8; 12]) -> Self {
        let [p0, p1, p2, p3,
             f0, f1, f2, f3,
             v0, v1, v2, v3] = buffer;
        let pid = i32::from_ne_bytes([p0, p1, p2, p3]);
        let fid = i32::from_ne_bytes([f0, f1, f2, f3]);
        let val = f32::from_ne_bytes([v0, v1, v2, v3]);
        Self { pid, fid, val }
    }
}
