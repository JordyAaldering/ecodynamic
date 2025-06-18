mod config;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::{mem, process};
use std::os::unix::net::{UnixListener, UnixStream};

use clap::Parser;
use config::Config;
use controller::*;
use rapl_energy::Constraint;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

fn handle_client(mut stream: UnixStream, config: Config, power_limit_old: u64) -> io::Result<()> {
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
                    .or_insert_with(|| (Vec::with_capacity(config.letterbox_size), config.build(req, power_limit_old)));

                let (global_demand, local_demand) = controller.next_demand();

                if global_demand.power_limit_uw > 0 {
                    set_power_limit(global_demand.power_limit_uw);
                }

                // Write to stream
                debug_println!("Send: {:?}", local_demand);
                let buf: [u8; LocalDemand::SIZE] = local_demand.to_bytes();
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

fn set_power_limit(power_limit_uw: u64) {
    debug_println!("Set power limit to {}", power_limit_uw);
    // long-term power limit
    Constraint::now(0, 0, None).map(|mut c| c.set_power_limit_uw(power_limit_uw));
    // short-term power limit
    Constraint::now(1, 0, None).map(|mut c| c.set_power_limit_uw(power_limit_uw));
}

fn main() -> io::Result<()> {
    let config = Config::parse();
    println!("{:?}", config);

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        debug_println!("Closing existing socket at {}", MTD_LETTERBOX_PATH);
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    debug_println!("Server listening on {}", MTD_LETTERBOX_PATH);

    let power_limit_old = Constraint::now(0, 0, None).map_or(0, |c| c.power_limit_uw);

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(move || {
        set_power_limit(power_limit_old);
        debug_println!("Closing socket at {}", MTD_LETTERBOX_PATH);
        let _ = fs::remove_file(MTD_LETTERBOX_PATH);
        process::exit(0);
    }).unwrap();

    if config.single {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, config, power_limit_old)?,
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    } else {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config_clone = config.clone();
                    std::thread::spawn(move || {
                        handle_client(stream, config_clone, power_limit_old)
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    set_power_limit(power_limit_old);
    debug_println!("Closing socket at {}", MTD_LETTERBOX_PATH);
    fs::remove_file(MTD_LETTERBOX_PATH)?;

    Ok(())
}
