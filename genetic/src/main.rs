use std::{io::{self, BufReader}, os::unix::net::UnixStream, process, sync::{LazyLock, Mutex, atomic}, thread};

use clap::Parser;
use ecodynamic_core::*;
use rapl_energy::Rapl;

use genetic::*;

static RAPL: LazyLock<Option<Mutex<Rapl>>> = LazyLock::new(|| {
    let rapl = Rapl::new(false);
    log::trace!("RAPL interface: {:?}", rapl);
    rapl.map(Mutex::new)
});

static THREAD_UTILIZATION: atomic::AtomicU16 = atomic::AtomicU16::new(0);

#[derive(Clone, Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,
    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    pub idle_power: f32,
    /// Size of the letterbox/population for each task.
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,
    /// Controller and hardware capabilities.
    #[clap(flatten)]
    pub ctx: ServerCapabilities,
    /// Genetic controller configuration.
    #[command(flatten)]
    pub config: GeneticConfig,
}

fn handle_client(
    mut stream: UnixStream,
    idle_power: f32,
    letterbox_size: usize,
    config: GeneticConfig,
    ctx: ServerCapabilities,
    hw: HardwareCapabilities,
) -> io::Result<()> {
    let mut rdr = BufReader::new(stream.try_clone()?);

    // First message must be the application's capabilities
    let app = socket::accept(&mut rdr)?;
    let capabilities = Capabilities { app: &app, ctx: &ctx, hw: &hw };

    let mut tasks = ApplicationContext::new(
        letterbox_size,
        || GeneticController::new(letterbox_size, &config, capabilities),
    );

    let mut last_thread_count = 0;

    loop {
        match socket::read(&mut rdr) {
            Ok(socket::Response::Request(request)) => {
                let demand = tasks.request(&request);

                let (index, controller) = tasks.current(&request);
                controller.store_state(index, State {
                    thread_utilization: THREAD_UTILIZATION.load(atomic::Ordering::Relaxed),
                    powercap_uw: 0,
                });

                // Must be run after get_demand, because the controller tracks the number of threads in use
                THREAD_UTILIZATION.fetch_add(demand.num_threads, atomic::Ordering::Relaxed);
                last_thread_count = demand.num_threads;

                set_powercap(demand.powercap_pct, hw.max_power_uw);

                socket::write(&mut stream, &demand)?;
            }
            Ok(socket::Response::Sample(mut sample)) => {
                // The task is over, so we can subtract the thread count from the global count
                // Must be run before push_sample, because the controller tracks the number of threads in use
                THREAD_UTILIZATION.fetch_sub(last_thread_count, atomic::Ordering::Relaxed);
                last_thread_count = 0;

                // Subtract idle energy
                sample.energy -= idle_power * sample.runtime;
                sample.energy = sample.energy.max(f32::EPSILON);
                tasks.push(sample);
            }
            Ok(socket::Response::Disconnect) => {
                // Before exiting, ensure the thread utilization is reset
                THREAD_UTILIZATION.fetch_sub(last_thread_count, atomic::Ordering::Relaxed);
                return Ok(());
            }
            Err(e) => {
                // Before aborting, ensure the thread utilization is reset
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
            log::debug!("Max power: {}uW", max_power_uw);
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

fn set_powercap(powercap_pct: f32, max_power_uw: u64) {
    if let Some(mut rapl) = RAPL.as_ref().map(|x| x.lock().unwrap()) {
        let powercap = (powercap_pct * max_power_uw as f32).round() as u64;
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

    let Args {
        once,
        idle_power,
        letterbox_size,
        ctx,
        config,
    } = Args::parse();

    // TODO: number of available cores assumed to be 8 for now
    let available_threads = 8;
    let max_power_uw = find_max_power_uw();
    let hw = HardwareCapabilities { available_threads, max_power_uw };

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        reset_default_power_limit().unwrap();
        socket::close().unwrap();
        process::exit(0);
    }).unwrap();

    if once {
        let stream = listener.incoming().next().unwrap()?;
        handle_client(stream, idle_power, letterbox_size, config, ctx, hw)?;
    } else {
        for stream in listener.incoming().map_while(Result::ok) {
            let config = config.clone();
            let ctx = ctx.clone();
            let hw = hw.clone();
            thread::spawn(move || {
                handle_client(stream, idle_power, letterbox_size, config, ctx, hw).unwrap()
            });
        }
    }

    reset_default_power_limit()?;
    socket::close()
}
