use std::mem;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

use letterbox::{Incoming, Outgoing};

const SOCKET_PATH: &str = "/tmp/mtdynamic_letterbox";

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    let pid = std::process::id() as i32;
    let val = std::env::args().collect::<Vec<_>>()[1].parse::<f32>().unwrap();

    // Write to stream
    let incoming = Incoming { pid, fid: 0, val };
    println!("Send: {:?}", incoming);
    stream.write_all(&incoming.to_bytes())?;

    // Read from stream
    let mut buffer = [0u8; mem::size_of::<Outgoing>()];
    stream.read_exact(&mut buffer)?;
    let outgoing = Outgoing::from(buffer);
    println!("Recv: {:?}", outgoing);

    Ok(())
}
