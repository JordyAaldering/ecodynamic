mod letterbox;

use std::collections::HashMap;
use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;

use controller::delta_controller::Controller;
use letterbox::Letterbox;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/rust_unix_socket";

    // Remove any existing socket file
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }

    // Create a listener
    let listener = UnixListener::bind(socket_path)?;
    println!("Server listening on {}", socket_path);

    let mut letterbox: Letterbox<10> = Letterbox::default();
    let mut controllers: HashMap<(i32, i32), Controller> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");

                let mut buffer = [0u8; 3 * std::mem::size_of::<i32>()];

                // Read from stream
                stream.read_exact(&mut buffer)?;
                let pid = i32::from_ne_bytes(buffer[0..4].try_into().unwrap());
                let fid = i32::from_ne_bytes(buffer[0..4].try_into().unwrap());
                let value = i32::from_ne_bytes(buffer[0..4].try_into().unwrap());
                println!("Received: ({}, {}) -> {}", pid, fid, value);

                // Update letterbox
                let samples = letterbox.update(pid, fid, value);

                // Lookup controller
                let controller = if let Some(controller) = controllers.get_mut(&(pid, fid)) {
                    controller
                } else {
                    let controller = Controller::new(16);
                    controllers.insert((pid, fid), controller);
                    controllers.get_mut(&(pid, fid)).unwrap()
                };

                if let Some(samples) = samples {
                    controller.adjust_threads(samples.map(|x| x as f32).to_vec());
                }

                let threads = controller.threads();

                // Write to stream
                stream.write_all(&threads.to_ne_bytes())?;
                println!("Set thread-count of ({}, {}) to {}", pid, fid, threads);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    fs::remove_file(socket_path)
}
