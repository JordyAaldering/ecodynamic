mod config;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::mem;
use std::os::unix::net::{UnixListener, UnixStream};

use clap::Parser;
use config::Config;
use controller::*;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

fn handle_client(mut stream: UnixStream, config: Config, client_id: usize) -> io::Result<()> {
    let mut lbs: HashMap<i32, (Vec<Sample>, Box<dyn Controller>)> = HashMap::new();

    let mut buffer = [0u8; Sample::SIZE];

    let mut log = if let Some(path) = &config.log_path {
        let path = path.join(format!("client{:02}.csv", client_id));
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

                if let Some(w) = &mut log {
                    w.write_fmt(format_args!("{},{},{},{},{}\n",
                        sample.region_uid,
                        controller.num_threads(),
                        sample.runtime,
                        sample.usertime,
                        sample.energy)
                    )?;
                }

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

    // Check if log directory exists
    if let Some(path) = &config.log_path {
        let path = fs::canonicalize(path)?;
        println!("Writing logs to {:?}", path);
    }

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    if config.single {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, config, 0)?,
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    } else {
        let mut client_count: usize = 0;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    client_count += 1;
                    let config_clone = config.clone();
                    std::thread::spawn(move || {
                        handle_client(stream, config_clone, client_count)
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    // Remove socket file
    fs::remove_file(MTD_LETTERBOX_PATH)?;

    Ok(())
}
