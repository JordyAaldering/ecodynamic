use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/rust_unix_socket";

    let pid = std::process::id() as i32;
    let fid = 0i32;

    let mut stream = UnixStream::connect(socket_path)?;

    let number = std::env::args().collect::<Vec<_>>()[1].parse::<i32>().unwrap();

    // Write 'measurement' to stream
    let numbers = [pid, fid, number];
    println!("Sending: {:?}", numbers);
    let mut buffer = Vec::new();
    for &num in &numbers {
        buffer.extend_from_slice(&num.to_ne_bytes());
    }
    stream.write_all(&buffer)?;

    // Read thread-count from stream
    let mut buffer = [0u8; std::mem::size_of::<i32>()];
    stream.read_exact(&mut buffer)?;
    let received_number = i32::from_ne_bytes(buffer);
    println!("Received: {}", received_number);

    Ok(())
}
