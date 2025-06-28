mod config;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::sync::{LazyLock, Mutex};
use std::{mem, process};
use std::os::unix::net::{UnixListener, UnixStream};

use clap::Parser;
use config::Config;
use controller::*;
use rapl_energy::Rapl;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

static RAPL: LazyLock<Mutex<Rapl>> = LazyLock::new(|| {
    let rapl = Rapl::now(false).expect("RAPL interface not found");
    println!("Found RAPL interface: {:?}", rapl);
    Mutex::new(rapl)
});

fn handle_client(mut stream: UnixStream, config: Config) -> io::Result<()> {
    let mut lbs: HashMap<i32, (Vec<Sample>, Box<dyn Controller>)> = HashMap::new();

    let mut buffer = [0u8; Sample::SIZE];

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let Request { region_uid, .. } = Request::from(buf);
                debug_println!("Read: {:?}", region_uid);

                // Update letterbox
                let (_, controller) = lbs.entry(region_uid)
                    .or_insert_with(|| (Vec::with_capacity(config.letterbox_size), config.build()));

                let (global_demand, local_demand) = controller.next_demand();

                set_power_limit(global_demand.power_limit_pct);

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

fn set_power_limit(power_limit_pct: f32) {
    let mut rapl = RAPL.lock().unwrap();
    for package in &mut rapl.packages {
        for constraint in &mut package.constraints {
            if let Some(max_power_uw) = constraint.max_power_uw {
                constraint.set_power_limit_uw((max_power_uw as f32 * power_limit_pct) as u64);
            } else {
                eprintln!("No max_power_uw found for constraint")
            }
        }
    }
}

fn reset_default_power_limit() {
    let mut rapl = RAPL.lock().unwrap();
    for package in &mut rapl.packages {
        for constraint in &mut package.constraints {
            if let Some(max_power_uw) = constraint.max_power_uw {
                constraint.set_power_limit_uw(max_power_uw);
            } else {
                eprintln!("No max_power_uw found for constraint")
            }
        }
    }
}

fn main() -> io::Result<()> {
    let config = Config::parse();
    println!("{:?}", config);

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        print!("Closing existing socket at {}", MTD_LETTERBOX_PATH);
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(move || {
        reset_default_power_limit();
        print!("Closing socket at {}", MTD_LETTERBOX_PATH);
        let _ = fs::remove_file(MTD_LETTERBOX_PATH);
        process::exit(0);
    }).unwrap();

    if config.once {
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

    reset_default_power_limit();
    println!("Closing socket at {}", MTD_LETTERBOX_PATH);
    fs::remove_file(MTD_LETTERBOX_PATH)?;

    Ok(())
}
