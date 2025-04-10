use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, Read, Write};
use std::{fs, mem};

use controller::*;
use letterbox::*;

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    #[cfg(feature = "corridor")]
    let mut letterbox: Letterbox<CorridorController> = Letterbox::new(|req| CorridorController::new(req.max_threads));
    #[cfg(feature = "delta")]
    let mut letterbox: Letterbox<DeltaController> = Letterbox::new(|req| DeltaController::new(req.max_threads));
    #[cfg(feature = "genetic")]
    let mut letterbox: Letterbox<GeneticController> = Letterbox::new(|req| GeneticController::new(req.max_threads, 20, 0.5, 0.25));

    const READREQ_SIZE: usize = mem::size_of::<Request>();
    const SAMPLE_SIZE: usize = mem::size_of::<Sample>();
    const DEMAND_SIZE: usize = mem::size_of::<Demand>();
    let mut buffer = [0u8; SAMPLE_SIZE];

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(READREQ_SIZE) => {
                let buf: [u8; READREQ_SIZE] = buffer[0..READREQ_SIZE].try_into().unwrap();
                let req = Request::from(buf);
                println!("Read: {:?}", req);

                // Update letterbox
                let demand = letterbox.read(req);

                // Write to stream
                println!("Send: {:?}", demand);
                let buf: [u8; DEMAND_SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(SAMPLE_SIZE) => {
                let sample = Sample::from(buffer);
                println!("Recv: {:?}", sample);
                letterbox.update(sample);
            }
            Ok(n) => {
                eprintln!("Invalid message size: {}", n);
                break;
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
