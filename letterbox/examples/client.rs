use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

const SOCKET_PATH: &str = "/tmp/mtdynamic_letterbox";

fn main() -> std::io::Result<()> {
    let pid = std::process::id() as i32;
    let fid = 0i32;

    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    let value = std::env::args().collect::<Vec<_>>()[1].parse::<f32>().unwrap();

    // Write 'measurement' to stream
    println!("Sending: ({}, {}) -> {:?}", pid, fid, value);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&pid.to_ne_bytes());
    buffer.extend_from_slice(&fid.to_ne_bytes());
    buffer.extend_from_slice(&value.to_ne_bytes());
    stream.write_all(&buffer)?;

    // Read thread-count from stream
    let mut buffer = [0u8; std::mem::size_of::<i32>()];
    stream.read_exact(&mut buffer)?;
    let received_number = i32::from_ne_bytes(buffer);
    println!("Received: {}", received_number);

    Ok(())
}
