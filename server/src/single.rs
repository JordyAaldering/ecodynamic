mod config;

use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};

use clap::Parser;
use config::*;
use letterbox::*;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[static_init::dynamic]
static CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::parse()));

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    let mut letterbox = Letterbox::new(|req| ControllerType::build(CONFIG.clone(), req));

    let mut buffer = [0u8; Sample::SIZE];

    let mut log = if let Some(path) = &CONFIG.lock().unwrap().log_path {
        println!("Creating log file at {:?}", path);
        let file = File::create_new(path)?;
        let mut w = BufWriter::new(file);
        w.write("uid,threads,runtime,usertime,energy\n".as_bytes())?;
        Some(w)
    } else {
        None
    };

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let req = Request::from(buf);
                debug_println!("Read: {:?}", req);

                // Update letterbox
                let num_threads = letterbox.try_get_demand(req);
                let demand = Demand { num_threads };

                // Write to stream
                debug_println!("Send: {:?}", demand);
                let buf: [u8; Demand::SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(Sample::SIZE) => {
                let sample = Sample::from(buffer);

                debug_println!("Recv: {:?}", sample);

                if let Some(w) = &mut log {
                    w.write_fmt(format_args!("{},{},{},{},{}\n",
                        sample.region_uid,
                        letterbox.get_demand(sample.region_uid),
                        sample.runtime,
                        sample.usertime,
                        sample.energy)
                    )?;
                }

                let uid = sample.region_uid;
                let score = CONFIG.lock().unwrap().score_function.score(sample);
                letterbox.update(uid, score);
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
    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    let stream = listener.incoming().next().unwrap();
    match stream {
        Ok(stream) => handle_client(stream)?,
        Err(e) => eprintln!("Connection failed: {}", e),
    }

    println!("Server shutting down");
    fs::remove_file(MTD_LETTERBOX_PATH)?;
    Ok(())
}
