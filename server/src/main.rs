use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, Read, Write};
use std::{fs, mem};

use controller::*;
use letterbox::*;

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    #[cfg(feature = "corridor")]
    let mut letterbox: Letterbox<CorridorController> = Letterbox::new(|s| CorridorController::new(s.max_threads));
    #[cfg(feature = "delta")]
    let mut letterbox: Letterbox<DeltaController> = Letterbox::new(|s| DeltaController::new(s.max_threads));
    #[cfg(feature = "genetic")]
    let mut letterbox: Letterbox<GeneticController> = Letterbox::new(|s| GeneticController::new(s.max_threads, 20, 0.5, 0.25));

    const REGION_SIZE: usize = mem::size_of::<i32>();
    const SAMPLE_SIZE: usize = mem::size_of::<Sample>();
    const DEMAND_SIZE: usize = mem::size_of::<Demand>();
    let mut buffer = [0u8; SAMPLE_SIZE];

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(REGION_SIZE) => {
                let [i0, i1, i2, i3, ..] = buffer;
                let region_uid = i32::from_ne_bytes([i0, i1, i2, i3]);
                println!("Read: {:?}", region_uid);

                // Update letterbox
                let demand = letterbox.read(region_uid);

                // Write to stream
                println!("Send: {:?}", demand);
                let buf: [u8; DEMAND_SIZE] = demand.unwrap_or_default().to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(SAMPLE_SIZE) => {
                let sample = Sample::from(buffer);
                println!("Recv: {:?}", sample);

                // Update letterbox
                let demand = letterbox.update(sample);

                // Write to stream
                println!("Send: {:?}", demand);
                let buf: [u8; DEMAND_SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(n) => {
                eprintln!("Invalid message size: {}", n);
            }
            Err(e) => {
                eprintln!("Client disconnected: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    unreachable!()
}
