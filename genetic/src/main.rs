mod capabilities;
mod chromosome;
mod controller;
mod gene;
pub(crate) mod knob;

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    os::unix::net::UnixStream,
    process,
    sync::{LazyLock, Mutex, atomic},
    thread,
};

use clap::Parser;
use ecodynamic_api::*;
use ecodynamic_core::*;
use rapl_energy::Rapl;

use controller::*;

use crate::capabilities::{Capabilities, HardwareCapabilities, ServerCapabilities, State};

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::new(false);
    log::trace!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

static THREAD_UTILIZATION: atomic::AtomicU16 = atomic::AtomicU16::new(0);

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,
    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    pub idle_power: f32,
    /// Controller and hardware capabilities.
    #[clap(flatten)]
    pub ctx: ServerCapabilities,
    /// Controller type.
    #[command(flatten)]
    pub config: Config,
}

fn handle_client(mut stream: UnixStream, args: Args, hw: HardwareCapabilities) -> io::Result<()> {
    let mut lbs: HashMap<i32, GeneticController> = HashMap::new();
    let mut rdr = BufReader::new(stream.try_clone()?);
    let mut line = String::new();

    // First message must be a capabilities broadcast from the client
    rdr.read_line(&mut line)?;
    let app: AppCapabilities = serde_json::from_str(line.trim_end())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Expected capabilities: {e}")))?;
    log::debug!("Client capabilities: {:?}", app);
    let capabilities = Capabilities::new(&app, &args.ctx, &hw);

    let mut last_thread_count = 0;

    loop {
        match socket::response(&mut rdr) {
            Ok(socket::Response::Request(request)) => {
                let controller = lbs.entry(request.region_uid)
                    .or_insert_with(|| {
                        log::info!("Generating controller for request {}", request.region_uid);
                        GeneticController::new(&args.config, capabilities)
                    });

                let mut demand = controller.get_demand();
                controller.store_state(State {
                    thread_utilization: THREAD_UTILIZATION.load(atomic::Ordering::Relaxed),
                    powercap_uw: 0,
                });
                demand.ensure_threads(capabilities.max_threads());

                // Must be run after get_demand, because the controller tracks the number of threads in use
                let num_threads = demand.num_threads(capabilities.max_threads());
                THREAD_UTILIZATION.fetch_add(num_threads, atomic::Ordering::Relaxed);
                last_thread_count = num_threads;

                let powercap = demand.powercap(capabilities.max_power_uw());
                set_powercap(powercap);

                socket::write(&mut stream, &demand)?;
            }
            Ok(socket::Response::Sample(mut sample)) => {
                // The region is over, so we can subtract the thread count from the global count
                // Must be run before push_sample, because the controller tracks the number of threads in use
                THREAD_UTILIZATION.fetch_sub(last_thread_count, atomic::Ordering::Relaxed);
                last_thread_count = 0;

                // Subtract idle energy
                sample.energy -= args.idle_power * sample.runtime;
                sample.energy = sample.energy.max(f32::EPSILON);

                lbs.get_mut(&sample.region_uid)
                    .expect("Received sample for region that has not yet been instantiated")
                    .push_sample(sample);
            }
            Ok(socket::Response::Disconnect) => {
                // Before exiting, ensure the thread utilization is reset
                THREAD_UTILIZATION.fetch_sub(last_thread_count, atomic::Ordering::Relaxed);
                return Ok(());
            }
            Err(e) => {
                // Before exiting, ensure the thread utilization is reset
                THREAD_UTILIZATION.fetch_sub(last_thread_count, atomic::Ordering::Relaxed);
                return Err(e)
            }
        }
    }
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

fn set_powercap(powercap: u64) {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        for package in &mut rapl.packages {
            if package.constraints.is_empty() {
                log::warn!("Skipping package {} without power constraints", package.name);
                continue;
            }

            let long_term = &mut package.constraints[0];

            log::trace!("Setting power limit for {} to {}uW",
                long_term.name.as_deref().unwrap_or("<unknown>"), powercap);
            if let Err(e) = long_term.set_power_limit_uw(powercap) {
                log::error!("Failed to set power limit for {}: {}",
                    long_term.name.as_deref().unwrap_or("<unknown>"), e);
            }

            if let Some(short_term) = package.constraints.get_mut(1) {
                log::trace!("Setting power limit for {} to {}uW",
                    short_term.name.as_deref().unwrap_or("<unknown>"), powercap);
                if let Err(e) = short_term.set_power_limit_uw(powercap) {
                    log::error!("Failed to set power limit for {}: {}",
                        short_term.name.as_deref().unwrap_or("<unknown>"), e);
                }
            }
        }
    }
}

fn reset_default_power_limit() -> io::Result<()> {
    if let Some(x) = RAPL.as_ref() {
        if let Ok(mut rapl) = x.lock() {
            rapl.reset_power_limits(false)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    env_logger::init();

    let args = Args::parse();
    log::trace!("Args: {args:?}");

    // TODO: number of available cores assumed to be 8 for now
    let available_threads = 8;
    let max_power_uw = find_max_power_uw();
    let hw = HardwareCapabilities::new(available_threads, max_power_uw);

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        reset_default_power_limit().unwrap();
        socket::close().unwrap();
        process::exit(0);
    }).unwrap();

    if args.once {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, args, hw).unwrap(),
            Err(e) => log::error!("Connection failed: {}", e),
        }
    } else {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let args = args.clone();
                    let hw = hw.clone();
                    thread::spawn(move || {
                        handle_client(stream, args, hw).unwrap()
                    });
                }
                Err(e) => log::error!("Connection failed: {}", e),
            }
        }
    }

    reset_default_power_limit()?;
    socket::close()
}
