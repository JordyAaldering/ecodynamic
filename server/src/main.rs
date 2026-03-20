mod config;

use std::{collections::HashMap, fs, io::{self, Read, Write}, mem, os::unix::net::{UnixListener, UnixStream}, process::{Command, exit}, sync::{LazyLock, Mutex}, thread};

use clap::Parser;
use controller::*;
use rapl_energy::Rapl;

use crate::config::Args;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::new(false);
    println!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

fn handle_client(mut stream: UnixStream, config: Args) -> io::Result<()> {
    let mut lbs: HashMap<i32, (Vec<Sample>, Box<dyn Controller>)> = HashMap::new();

    let mut buffer = [0u8; Sample::SIZE];

    loop {
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
                let sample = Sample::from(buffer);
                debug_println!("Recv: {:?}", sample);

                let (samples, controller) = lbs.get_mut(&sample.region_uid).unwrap();

                samples.push(sample);
                if samples.len() >= config.letterbox_size {
                    let mut swap = Vec::with_capacity(config.letterbox_size);
                    mem::swap(samples, &mut swap);

                    // Subtract idle
                    for sample in &mut swap {
                        sample.energy -= config.idle_power * sample.runtime;
                        sample.energy = sample.energy.min(0.0);
                    }

                    controller.evolve(swap);
                }
            }
            Err(e) => {
                println!("Client disconnected");
                return Err(e);
            }
            Ok(0) => {
                println!("Client disconnected");
                return Ok(());
            }
            Ok(n) => {
                eprintln!("Invalid message size: {}", n);
                continue;
            }
        }
    }
}

fn set_power_limit(power_limit_pct: f32) {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        for package in &mut rapl.packages {
            // In some cases power_limit_uw is 0, use the long-term power limit as a fallback
            let long_term = package.constraints.iter()
                .find(|c| c.name.as_ref().is_some_and(|s| s == "long_term"))
                .map(|c| c.max_power_uw.expect("long_term constraint must have max_power_uw"));

            for constraint in &mut package.constraints {
                if let Some(max_power_uw) = constraint.max_power_uw.or(long_term) {
                    let limit = (max_power_uw as f32 * power_limit_pct) as u64;
                    if let Err(e) = constraint.set_power_limit_uw(limit) {
                        eprintln!("Failed to set power limit for {}: {}", constraint.name.as_deref().unwrap_or("unknown"), e);
                    }
                } else {
                    eprintln!("No max_power_uw found for constraint {}", constraint.name.as_deref().unwrap_or("unknown"));
                }
            }
        }
    }

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
                    if let Err(e) = constriant.set_power_limit_uw(limit) {
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
        if let Err(e) = rapl.reset_power_limits(false) {
            eprintln!("Failed to reset power limits: {}", e)
        }
    }
}

fn main() {
    let config = Args::parse();

    let listener = open_socket();

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        close_socket();
        exit(0);
    }).unwrap();

    if let Some(prog) = &config.cmd {
        println!("Running controller only for: {}", prog);
        Command::new(prog).spawn().unwrap();

        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, config).unwrap(),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    } else {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config_clone = config.clone();
                    thread::spawn(move || {
                        handle_client(stream, config_clone).unwrap()
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    close_socket();
}

fn open_socket() -> UnixListener {
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        println!("Closing previous socket: {}", MTD_LETTERBOX_PATH);
        fs::remove_file(MTD_LETTERBOX_PATH).expect("Could not close socket");
    }

    println!("Creating socket: {}", MTD_LETTERBOX_PATH);
    UnixListener::bind(MTD_LETTERBOX_PATH).expect("Could not create socket")
}

fn close_socket() {
    reset_default_power_limit();
    println!("Closing socket: {}", MTD_LETTERBOX_PATH);
    fs::remove_file(MTD_LETTERBOX_PATH).expect("Could not close socket");
}
