use std::collections::HashMap;
use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/rust_unix_socket";

    // Remove any existing socket file
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }

    // Create a listener
    let listener = UnixListener::bind(socket_path)?;

    println!("Server listening on {}", socket_path);

    let mut counters: HashMap<i32, i32> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");

                let mut buffer = [0u8; 4]; // Buffer to read the integer
                stream.read_exact(&mut buffer)?;
                let number = i32::from_ne_bytes(buffer);
                println!("Received: {}", number);

                let counter = counters.get(&number).map_or(1, |x| x + 1);

                std::thread::sleep(std::time::Duration::from_secs(2));

                counters.insert(number, counter);

                stream.write_all(&counter.to_ne_bytes())?;
                println!("Sent: {}", counter);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
