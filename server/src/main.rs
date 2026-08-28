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

    /// Enable thread control.
    #[arg(long("thread-control"))]
    pub do_thread_control: bool,

    /// Enable thread pinning control.
    #[arg(long("pinning-control"))]
    pub do_pinning_control: bool,

    /// Enable power limit control.
    #[arg(long("power-control"))]
    pub do_power_control: bool,
    /// Minimum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.1)]
    pub min_power: f32,
    /// Maximum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 1.0)]
    pub max_power: f32,

    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ControllerType {
    /// Genetic algorithm approach.
    Genetic(GeneticSettings),
    /// Algorithm based on a performance corridor.
    Corridor(CorridorSettings),
    /// Algorithm based on deltas between runs.
    Delta(DeltaSettings),
    /// Continuously oscillates between configurations.
    Oscilating,
    /// Always returns the same configuration.
    Fixed,
}

pub enum ControllerImpl<'a> {
    Genetic(GeneticController<'a>),
    Corridor(CorridorController),
    Delta(DeltaController),
    Oscilating(OscilatingController),
    Fixed(FixedController),
}

impl<'a> ControllerImpl<'a> {
    fn build(args: &'a Args, capabilities: &'a Capabilities) -> Self {
        match &args.controller {
            ControllerType::Genetic(settings) => Self::Genetic(GeneticController::new(settings, capabilities)),
            ControllerType::Corridor(settings) => Self::Corridor(CorridorController::new(settings, capabilities)),
            ControllerType::Delta(settings) => Self::Delta(DeltaController::new(settings, capabilities)),
            ControllerType::Oscilating => Self::Oscilating(OscilatingController::new(capabilities)),
            ControllerType::Fixed => Self::Fixed(FixedController::new(capabilities)),
        }
    }

    fn get_demand(&mut self) -> Demand {
        match self {
            Self::Genetic(controller) => controller.get_demand(),
            Self::Corridor(controller) => controller.get_demand(),
            Self::Delta(controller) => controller.get_demand(),
            Self::Oscilating(controller) => controller.get_demand(),
            Self::Fixed(controller) => controller.get_demand(),
        }
    }

    fn push(&mut self, sample: Sample) {
        match self {
            Self::Genetic(controller) => controller.push(sample),
            Self::Corridor(controller) => controller.push(sample),
            Self::Delta(controller) => controller.push(sample),
            Self::Oscilating(controller) => controller.push(sample),
            Self::Fixed(controller) => controller.push(sample),
        }
    }
}

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::new(false);
    log::trace!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

fn handle_client(mut stream: UnixStream, args: Args) -> io::Result<()> {
    let mut lbs: HashMap<i32, ControllerImpl> = HashMap::new();
    let mut rdr = BufReader::new(stream.try_clone()?);
    let mut line = String::new();

    // First message must be a capabilities broadcast from the client
    rdr.read_line(&mut line)?;
    let app_capabilities: CapabilitiesResp = serde_json::from_str(line.trim_end())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Expected capabilities: {e}")))?;
    let capabilities = Capabilities {
        pid: app_capabilities.pid,
        max_threads: app_capabilities.max_threads,
        thread_control: args.do_thread_control,
        pinning_control: args.do_pinning_control,
        power_control: args.do_power_control,
        min_power: args.min_power,
        max_power: args.max_power,
    };
    log::debug!("Client capabilities: {:?}", capabilities);

    let mut last_thread_count = 0;

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

                    // The region is over, so we can subtract the thread count from the global count
                    // Must be run before push_sample, because the controller tracks the number of threads in use
                    STATE.remove_threads(last_thread_count);
                    last_thread_count = 0;

                    // Subtract idle energy
                    sample.energy -= args.idle_power * sample.runtime;
                    sample.energy = sample.energy.max(f32::EPSILON);

                    lbs.get_mut(&sample.region_uid)
                        .expect("Received sample for region that has not yet been instantiated")
                        .push(sample);
                } else if let Ok(request) = serde_json::from_str::<Request>(&line) {
                    log::trace!("GET: {:?}", request.region_uid);

                    let controller = lbs.entry(request.region_uid)
                        .or_insert_with(|| {
                            log::info!("Generating controller for request {}", request.region_uid);
                            ControllerImpl::build(&args, &capabilities)
                        });

                    let demand = controller.get_demand();
                    log::trace!("PUT: {:?}", demand);

                    // Must be run after get_demand, because the controller tracks the number of threads in use
                    STATE.add_threads(demand.num_threads);
                    last_thread_count = demand.num_threads;

                    set_power_limit(demand.powercap_pct);
                    write_json_line(&mut stream, &demand)?;
                } else {
                    // If the program aborted, it could be that the thread count was not yet reset
                    STATE.remove_threads(last_thread_count);

                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid JSON message: {line}"))
                    )
                }
            }
            Err(e) => {
                // If the program aborted, it could be that the thread count was not yet reset
                STATE.remove_threads(last_thread_count);

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

fn find_max_power_uw() -> u64 {
    if let Some(rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        let max_power_uw = rapl.packages.first()
            .and_then(|p| p.constraints.first())
            .and_then(|c| c.max_power_uw);
        if let Some(max_power_uw) = max_power_uw {
            log::info!("Max power: {}uW", max_power_uw);
            max_power_uw
        } else {
            log::warn!("RAPL does not provide max_power_uw; using 0uW");
            0
        }
    } else {
        log::warn!("RAPL not available; using 0uW as max power");
        0
    }
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

    // TODO: number of available cores assumed to be 8 for now
    HARDWARE.available_cores.set(8).expect("available_cores initialized twice");
    HARDWARE.max_power_uw.set(find_max_power_uw()).expect("max_power_uw initialized twice");

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
