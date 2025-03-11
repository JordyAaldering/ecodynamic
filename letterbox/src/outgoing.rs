use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct Outgoing {
    pub threads: i32,
}

const SIZE: usize = mem::size_of::<Outgoing>();

impl Outgoing {
    pub fn to_bytes(self) -> [u8; SIZE] {
        self.threads.to_ne_bytes()
    }
}

impl From<[u8; SIZE]> for Outgoing {
    fn from(buffer: [u8; SIZE]) -> Self {
        let threads = i32::from_ne_bytes(buffer);
        Self { threads }
    }
}
