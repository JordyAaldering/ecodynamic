#[repr(C)]
#[derive(Debug)]
pub struct Outgoing {
    pub threads: i32,
}

impl Outgoing {
    pub fn to_bytes(self) -> [u8; 4] {
        self.threads.to_ne_bytes()
    }
}

impl From<[u8; 4]> for Outgoing {
    fn from(buffer: [u8; 4]) -> Self {
        let threads = i32::from_ne_bytes(buffer);
        Self { threads }
    }
}
