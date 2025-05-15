mod config;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::{mem, process};
use std::os::unix::net::{UnixListener, UnixStream};

use clap::Parser;
use config::Config;
use controller::*;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

fn handle_client(mut stream: UnixStream, config: Config) -> io::Result<()> {
    let mut lbs: HashMap<i32, (Vec<Sample>, Box<dyn Controller>)> = HashMap::new();

    let mut buffer = [0u8; Sample::SIZE];

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let req = Request::from(buf);
                debug_println!("Read: {:?}", req);

                // Update letterbox
                let (_, controller) = lbs.entry(req.region_uid)
                    .or_insert_with(|| (Vec::with_capacity(config.letterbox_size), config.build(req)));
                let num_threads = controller.num_threads();
                let demand = Demand { num_threads };

                // Write to stream
                debug_println!("Send: {:?}", demand);
                let buf: [u8; Demand::SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(Sample::SIZE) => {
                let sample = Sample::from(buffer);

                debug_println!("Recv: {:?}", sample);

                let (samples, controller) = lbs.get_mut(&sample.region_uid).unwrap();

                samples.push(sample);
                if samples.len() >= config.letterbox_size {
                    let mut swap = Vec::with_capacity(config.letterbox_size);
                    mem::swap(samples, &mut swap);
                    controller.evolve(swap);
                }
            }
            Ok(0) => {
                println!("Client disconnected");
                break;
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
    let config = Config::parse();

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        debug_println!("Closing existing socket at {}", MTD_LETTERBOX_PATH);
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    debug_println!("Server listening on {}", MTD_LETTERBOX_PATH);

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(move || {
        debug_println!("Closing socket at {}", MTD_LETTERBOX_PATH);
        let _ = fs::remove_file(MTD_LETTERBOX_PATH);
        process::exit(0);
    }).unwrap();

    if config.single {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, config)?,
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    } else {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config_clone = config.clone();
                    std::thread::spawn(move || {
                        handle_client(stream, config_clone)
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    debug_println!("Closing socket at {}", MTD_LETTERBOX_PATH);
    fs::remove_file(MTD_LETTERBOX_PATH)?;

    Ok(())
}
