mod config;

use std::{collections::HashMap, fs, io::{self, Read, Write}, os::unix::net::{UnixListener, UnixStream}, process, sync::{LazyLock, Mutex}, thread};

use clap::Parser;
use controller::*;
use rapl_energy::Rapl;

use crate::config::Args;

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::new(false);
    log::trace!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

fn handle_client(mut stream: UnixStream, config: Args) -> io::Result<()> {
    let mut lbs: HashMap<i32, Box<dyn Controller>> = HashMap::new();
    let mut last_demand = LocalDemand { threads_pct: 1.0 };
    let mut buffer = [0u8; Sample::SIZE];

    loop {
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let Request { region_uid, .. } = Request::from(buf);
                log::trace!("GET: {:?}", region_uid);

                if let Some(controller) = lbs.get_mut(&region_uid) {
                    let (global_demand, local_demand) = controller.get_demand();

                    log::trace!("PUT: {:?} {:?}", global_demand, local_demand);
                    set_power_limit(global_demand.powercap_pct);
                    let buf = local_demand.to_bytes();
                    stream.write_all(&buf)?;
                    last_demand = local_demand;
                } else {
                    // Use the last-used configuration in an attempt to minimise configuration changes
                    // Note that changes may still occur, if multiple clients are connected.
                    log::trace!("PUT: {:?}", last_demand);
                    let buf = last_demand.to_bytes();
                    stream.write_all(&buf)?;
                }
            }
            Ok(Sample::SIZE) => {
                let mut sample = Sample::from(buffer);

                if sample.runtime < 0.01 && !lbs.contains_key(&sample.region_uid) {
                    // Ignore samples without a controller, where runtime is too short for accurate energy measurements
                    // In these cases, the overhead of adjusting the configuration outweighs the potential benefits
                    continue;
                }

                // Subtract idle power draw
                sample.energy -= config.idle_power * sample.runtime;
                sample.energy = sample.energy.max(f32::EPSILON);
                log::trace!("POST: {:?}", sample);

                // Push sample to the controller, which can cause it to `evolve'
                lbs.entry(sample.region_uid)
                    .or_insert_with(|| config.build_controller())
                    .push_sample(sample);
            }
            Err(e) => {
                log::info!("Client disconnected");
                return Err(e);
            }
            Ok(0) => {
                log::info!("Client disconnected");
                return Ok(());
            }
            Ok(n) => {
                log::error!("Invalid message size: {}", n);
                continue;
            }
        }
    }
}

fn set_power_limit(power_limit_pct: f32) {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        for package in &mut rapl.packages {
            let long_term = &mut package.constraints[0];
            let max_power_uw = long_term.max_power_uw.expect("long_term constraint must have max_power_uw");
            let limit = (max_power_uw as f32 * power_limit_pct) as u64;

            log::trace!("Setting power limit for {} to {}uW ({}% of max)",
                long_term.name.as_deref().unwrap_or("<unknown>"), limit, power_limit_pct * 100.0);
            if let Err(e) = long_term.set_power_limit_uw(limit) {
                log::error!("Failed to set power limit for {}: {}",
                    long_term.name.as_deref().unwrap_or("<unknown>"), e);
            }

            if let Some(short_term) = package.constraints.get_mut(1) {
                let max_power_uw = short_term.max_power_uw.map_or(max_power_uw, |c| if c > 0 { c } else { max_power_uw });
                let limit = (max_power_uw as f32 * power_limit_pct) as u64;

                log::trace!("Setting power limit for {} to {}uW ({}% of max)",
                    short_term.name.as_deref().unwrap_or("<unknown>"), limit, power_limit_pct * 100.0);
                if let Err(e) = short_term.set_power_limit_uw(limit) {
                    log::error!("Failed to set power limit for {}: {}",
                        short_term.name.as_deref().unwrap_or("<unknown>"), e);
                }
            }
        }
    }
}

fn reset_default_power_limit() {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        if let Err(e) = rapl.reset_power_limits(false) {
            log::error!("Failed to reset power limits: {}", e);
        }
    }
}

fn main() {
    env_logger::init();

    let config = Args::parse();
    log::trace!("Config: {:?}", config);

    let listener = open_socket();

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        close_socket();
        process::exit(0);
    }).unwrap();

    if config.once {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, config).unwrap(),
            Err(e) => log::error!("Connection failed: {}", e),
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
                Err(e) => log::error!("Connection failed: {}", e),
            }
        }
    }

    close_socket();
}

fn open_socket() -> UnixListener {
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        log::warn!("Closing previous socket: {}", MTD_LETTERBOX_PATH);
        fs::remove_file(MTD_LETTERBOX_PATH).expect("Could not close socket");
    }

    log::info!("Creating socket: {}", MTD_LETTERBOX_PATH);
    UnixListener::bind(MTD_LETTERBOX_PATH).expect("Could not create socket")
}

fn close_socket() {
    reset_default_power_limit();
    log::info!("Closing socket: {}", MTD_LETTERBOX_PATH);
    fs::remove_file(MTD_LETTERBOX_PATH).expect("Could not close socket");
}
