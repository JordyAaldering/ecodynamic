mod config;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::sync::{LazyLock, Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::{mem, process};
use std::os::unix::net::{UnixListener, UnixStream};

use clap::Parser;
use config::Config;
use controller::*;
use rapl_energy::Rapl;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::now(false);
    println!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

static IDLE_POWER: RwLock<f32> = RwLock::new(0.0);

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
                let buf: [u8; Demand::SIZE] = local_demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(Sample::SIZE) => {
                let mut sample = Sample::from(buffer);
                debug_println!("Recv: {:?}", sample);

                { // Subtract idle power draw from sample
                    let sample_power = sample.energy / (sample.runtime + f32::EPSILON);
                    let idle_power = *IDLE_POWER.read().unwrap();
                    if sample_power < idle_power && config.idle_power.is_none() {
                        // Power draw of the sample is less than automatically predetermined power draw, update idle power.
                        // We don't want to update the idle power draw if it was manually specified by the user,
                        // which is why we check whether config.idle_power does not have a value.
                        *IDLE_POWER.write().unwrap() = sample_power;
                    }

                    sample.energy -= idle_power * sample.runtime;
                    sample.energy = sample.energy.max(0.0);
                }

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
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        for package in &mut rapl.packages {
            // For some reason power_limit_uw is 0 for the short-term power limit, so we reuse the long-term limit.
            if let Some(max_power_uw) = package.constraints.get(0).and_then(|c| c.max_power_uw) {
                let limit = (max_power_uw as f32 * power_limit_pct) as u64;
                let e = package.constraints.get_mut(0).unwrap().set_power_limit_uw(limit);
                if let Err(e) = e {
                    eprintln!("{}", e);
                }

                if let Some(constriant) = package.constraints.get_mut(1) {
                    let e = constriant.set_power_limit_uw(limit);
                    if let Err(e) = e {
                        eprintln!("{}", e);
                    }
                }
            } else {
                eprintln!("No max_power_uw found for constraint")
            }
        }
    }
}

fn reset_default_power_limit() {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        for package in &mut rapl.packages {
            if let Some(max_power_uw) = package.constraints.get(0).and_then(|c| c.max_power_uw) {
                let e = package.constraints.get_mut(0).unwrap().set_power_limit_uw(max_power_uw);
                if let Err(e) = e {
                    eprintln!("{}", e);
                }

                if let Some(constriant) = package.constraints.get_mut(1) {
                    let e = constriant.set_power_limit_uw(max_power_uw);
                    if let Err(e) = e {
                        eprintln!("{}", e);
                    }
                }
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

    if let Some(idle_power) = config.idle_power {
        *IDLE_POWER.write().unwrap() = idle_power;
    } else if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        const N: usize = 60;
        println!("Measuring idle power draw ({N}s)");

        let mut min_power = f32::MAX;
        for _ in 0..N {
            rapl.reset();
            sleep(Duration::from_secs(1));
            let w = rapl.elapsed().into_values().sum();
            min_power = min_power.min(w);
        }

        *IDLE_POWER.write().unwrap() = min_power;
    } else {
        println!("Ignoring idle power draw because RAPL is not available");
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
