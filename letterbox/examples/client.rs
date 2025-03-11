use std::mem;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

use letterbox::{Incoming, Outgoing};

const SOCKET_PATH: &str = "/tmp/mtd_letterbox";

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    loop {
        // Write to stream
        let incoming = Incoming { max: 16, uid: 0, val: 42.37 };
        println!("Send: {:?}", incoming);
        stream.write_all(&incoming.to_bytes())?;

        // Read from stream
        let mut buffer = [0u8; mem::size_of::<Outgoing>()];
        stream.read_exact(&mut buffer)?;
        let outgoing = Outgoing::from(buffer);
        println!("Recv: {:?}", outgoing);

        sleep(Duration::from_secs(1));
    }
}
