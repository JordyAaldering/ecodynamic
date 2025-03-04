use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/rust_unix_socket";

    // Connect to the server
    let mut stream = UnixStream::connect(socket_path)?;

    let number = std::env::args().collect::<Vec<_>>()[1].parse::<i32>().unwrap();
    println!("Sending: {}", number);
    stream.write_all(&number.to_ne_bytes())?;

    let mut buffer = [0u8; 4]; // Buffer to read the response
    stream.read_exact(&mut buffer)?;

    let received_number = i32::from_ne_bytes(buffer);
    println!("Received: {}", received_number);

    Ok(())
}
