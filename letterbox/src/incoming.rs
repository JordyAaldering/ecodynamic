use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct Incoming {
    pub uid: i32,
    pub val: f32,
}

const SIZE: usize = mem::size_of::<Incoming>();

impl Incoming {
    pub fn to_bytes(self) -> [u8; SIZE] {
        let [u0, u1, u2, u3] = self.uid.to_ne_bytes();
        let [v0, v1, v2, v3] = self.val.to_ne_bytes();
        [u0, u1, u2, u3,
         v0, v1, v2, v3]
    }
}

impl From<[u8; SIZE]> for Incoming {
    fn from(buffer: [u8; SIZE]) -> Self {
        let [u0, u1, u2, u3,
             v0, v1, v2, v3] = buffer;
        let uid = i32::from_ne_bytes([u0, u1, u2, u3]);
        let val = f32::from_ne_bytes([v0, v1, v2, v3]);
        Self { uid, val }
    }
}
