use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader, Write},
    os::unix::net::{UnixListener, UnixStream},
    process,
    sync::{LazyLock, Mutex},
    thread,
};

use clap::{Parser, Subcommand};
use controller::*;
use rapl_energy::Rapl;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,

    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    pub idle_power: f32,

    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ControllerType {
    /// Genetic algorithm approach.
    Genetic(GeneticControllerConfig),
    /// Algorithm based on a performance corridor.
    Corridor(CorridorControllerConfig),
    /// Algorithm based on deltas between runs.
    Delta(DeltaControllerConfig),
    /// Continuously oscillates between the zero-capabilities and the given capabilities.
    Oscilating,
    /// Always returns the given capabilities.
    Fixed,
}

impl Args {
    pub fn build_controller(&self, caps: &Capabilities) -> Box<dyn Controller> {
        use ControllerType::*;
        match &self.controller {
            Genetic(config) => Box::new(GeneticController::new(config.clone(), caps)),
            Corridor(config) => Box::new(CorridorController::new(config.clone(), caps)),
            Delta(config) => Box::new(DeltaController::new(config.clone(), caps)),
            Oscilating => Box::new(OscilatingController::new(caps)),
            Fixed => Box::new(FixedController::new(caps)),
        }
    }
}

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::new(false);
    log::trace!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

fn handle_client(mut stream: UnixStream, config: Args) -> io::Result<()> {
    let mut lbs: HashMap<i32, Box<dyn Controller>> = HashMap::new();
    let mut rdr = BufReader::new(stream.try_clone()?);
    let mut line = String::new();

    // First message must be a capabilities broadcast from the client
    rdr.read_line(&mut line)?;
    let capabilities = serde_json::from_str(line.trim_end())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Expected capabilities: {e}")))?;
    log::debug!("Client capabilities: {:?}", capabilities);

    loop {
        line.clear();
        match rdr.read_line(&mut line) {
            Ok(0) => {
                log::info!("Client disconnected");
                return Ok(());
            }
            Ok(_) => {
                log::trace!("Received message: `{}`", line.trim_end());
                // Note that we must check for <Sample> first, because otherwise the message may be seen as a <Request>,
                // which happens when the request only contains the region, in which case the extra fields get ignored.
                if let Ok(mut sample) = serde_json::from_str::<Sample>(&line) {
                    log::trace!("POST: {:?}", sample);

                    // Subtract idle energy
                    sample.energy -= config.idle_power * sample.runtime;
                    sample.energy = sample.energy.max(f32::EPSILON);

                    let controller = lbs.entry(sample.region_uid)
                        .or_insert_with(|| {
                            // At this point a controller should already exist
                            log::warn!("Generating controller for sample {}", sample.region_uid);
                            config.build_controller(&capabilities)
                        });

                    controller.push_sample(sample);
                } else if let Ok(request) = serde_json::from_str::<Request>(&line) {
                    log::trace!("GET: {:?}", request.region_uid);

                    let controller = lbs.entry(request.region_uid)
                        .or_insert_with(|| {
                            log::info!("Generating controller for request {}", request.region_uid);
                            config.build_controller(&capabilities)
                        });

                    let demand = controller.get_demand();
                    log::trace!("PUT: {:?}", demand);
                    set_power_limit(demand.powercap_pct);
                    write_json_line(&mut stream, &demand)?;
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid JSON message: {line}"))
                    )
                }
            }
            Err(e) => {
                log::info!("Client disconnected");
                return Err(e);
            }
        }
    }
}

fn write_json_line<T: serde::Serialize>(stream: &mut UnixStream, message: &T) -> io::Result<()> {
    serde_json::to_writer(&mut *stream, message).map_err(io::Error::other)?;
    stream.write_all(b"\n")
}

fn set_power_limit(power_limit_pct: f32) {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        for package in &mut rapl.packages {
            if package.constraints.is_empty() {
                log::warn!("Skipping package {} without power constraints", package.name);
                continue;
            }

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
    if let Some(x) = RAPL.as_ref() {
        if let Ok(mut rapl) = x.lock() {
            if let Err(e) = rapl.reset_power_limits(false) {
                log::error!("Failed to reset power limits: {}", e);
            }
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
    if fs::metadata(LETTERBOX_PATH).is_ok() {
        log::warn!("Closing previous socket: {}", LETTERBOX_PATH);
        fs::remove_file(LETTERBOX_PATH).expect("Could not close socket");
    }

    log::info!("Creating socket: {}", LETTERBOX_PATH);
    UnixListener::bind(LETTERBOX_PATH).expect("Could not create socket")
}

fn close_socket() {
    reset_default_power_limit();
    log::info!("Closing socket: {}", LETTERBOX_PATH);
    fs::remove_file(LETTERBOX_PATH).expect("Could not close socket");
}
