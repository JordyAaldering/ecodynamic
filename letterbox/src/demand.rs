use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct Demand {
    pub threads: i32,
}

const SIZE: usize = mem::size_of::<Demand>();

impl Demand {
    pub fn to_bytes(self) -> [u8; SIZE] {
        self.threads.to_ne_bytes()
    }
}

impl From<[u8; SIZE]> for Demand {
    fn from(buffer: [u8; SIZE]) -> Self {
        let threads = i32::from_ne_bytes(buffer);
        Self { threads }
    }
}
