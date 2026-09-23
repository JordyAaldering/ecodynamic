use std::{io::{self, BufReader}, os::unix::net::UnixStream, process, sync::{LazyLock, Mutex, atomic}, thread};

use clap::Parser;
use ecodynamic_core::*;
use rapl_energy::Rapl;

use genetic::*;

static RAPL: LazyLock<Mutex<Rapl>> = LazyLock::new(|| {
    let rapl = Rapl::new(false).unwrap();
    log::trace!("Instantiated RAPL: {rapl:?}");
    Mutex::new(rapl)
});

/// Global system state, shared across all clients and threads.
static THREAD_UTILIZATION: atomic::AtomicU16 = atomic::AtomicU16::new(0);

// /// Global system state, shared across all clients and threads.
// static CURRENT_POWERCAP: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Clone, Parser)]
struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    once: bool,
    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    idle_power: f32,
    /// Size of the letterbox/population for each task.
    #[arg(short('s'), long, default_value_t = 20)]
    letterbox_size: usize,
    /// Genetic controller configuration.
    #[command(flatten)]
    config: GeneticConfig,
    /// Hardware capabilities, defined in a .env file or environment variables.
    #[command(flatten)]
    env: HardwareCapabilities,
}

fn handle_client(
    mut stream: UnixStream,
    idle_power: f32,
    letterbox_size: usize,
    config: GeneticConfig,
    env: HardwareCapabilities,
) -> io::Result<()> {
    let mut rdr = BufReader::new(stream.try_clone()?);

    // First message must be the application's capabilities
    let app = socket::accept(&mut rdr)?;

    let mut tasks = ApplicationContext::new(
        letterbox_size,
        || GeneticController::new(letterbox_size, &config, &app, &env),
    );

    let mut last_thread_count = 0;

    loop {
        match socket::read(&mut rdr) {
            Ok(socket::Response::Request(request)) => {
                let demand = tasks.request(&request);

                let (index, controller) = tasks.current(&request);
                controller.store_state(index, THREAD_UTILIZATION.load(atomic::Ordering::Relaxed));

                // Must be run after get_demand, because the controller tracks the number of threads in use
                THREAD_UTILIZATION.fetch_add(demand.num_threads, atomic::Ordering::Relaxed);
                last_thread_count = demand.num_threads;

                set_powercap(demand.powercap_pct, env.max_power_uw)?;

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

/// Set the power limit to the specified percentage of the maximum power limit.
fn set_powercap(powercap_pct: f32, max_power_uw: u64) -> io::Result<()> {
    let powercap = (powercap_pct * max_power_uw as f32).round() as u64;
    let mut rapl = RAPL.lock().unwrap();
    for package in &mut rapl.packages {
        for constraint in &mut package.constraints {
            constraint.set_power_limit_uw(powercap)?;
        }
    }
    Ok(())
}

/// Reset the power limits to their default values.
fn reset_power_limits() -> io::Result<()> {
    let mut rapl = RAPL.lock().unwrap();
    rapl.reset_power_limits(false)
}

fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let Args {
        once,
        idle_power,
        letterbox_size,
        config,
        env,
    } = Args::parse();

    log::debug!("{env:?}");

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        let _ = reset_power_limits();
        let _ = socket::close();
        process::exit(0);
    }).unwrap();

    if once {
        let stream = listener.incoming().next().unwrap()?;
        handle_client(stream, idle_power, letterbox_size, config, env)?;
    } else {
        for stream in listener.incoming().map_while(Result::ok) {
            let config = config.clone();
            let env = env.clone();
            thread::spawn(move || {
                handle_client(stream, idle_power, letterbox_size, config, env).unwrap()
            });
        }
    }

    reset_power_limits()?;
    socket::close()
}
