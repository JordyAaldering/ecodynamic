mod controller;

use std::{
    collections::HashMap,
    io::{self, BufReader},
    os::unix::net::UnixStream,
    process,
    thread,
};

use clap::Parser;
use ecodynamic_core::socket;

use crate::controller::{CorridorController, Config};

#[derive(Clone, Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,
    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    pub idle_power: f32,
    /// Controller type.
    #[command(flatten)]
    pub config: Config,
}

fn handle_client(mut stream: UnixStream, args: Args) -> io::Result<()> {
    let mut lbs: HashMap<i32, CorridorController> = HashMap::new();
    let mut rdr = BufReader::new(stream.try_clone()?);

    // First message must be the application's capabilities
    let capabilities = socket::accept(&mut rdr)?;

    loop {
        match socket::read(&mut rdr)? {
            socket::Response::Request(request) => {
                let controller = lbs.entry(request.task_id)
                    .or_insert_with(|| {
                        log::debug!("Generating controller for request {}", request.task_id);
                        CorridorController::new(&args.config, &capabilities)
                    });

                let demand = controller.get_demand();
                socket::write(&mut stream, &demand)?;
            }
            socket::Response::Sample(mut sample) => {
                // Subtract idle energy
                sample.energy -= args.idle_power * sample.runtime;
                sample.energy = sample.energy.max(f32::EPSILON);

                lbs.get_mut(&sample.region_uid)
                    .expect("Received sample for a task that has not yet been instantiated")
                    .push(sample);
            }
            socket::Response::Disconnect => {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        socket::close().unwrap();
        process::exit(0);
    }).unwrap();

    if args.once {
        let stream = listener.incoming().next().unwrap()?;
        handle_client(stream, args)?;
    } else {
        for stream in listener.incoming().map_while(Result::ok) {
            let args = args.clone();
            thread::spawn(move || {
                handle_client(stream, args).unwrap()
            });
        }
    }

    socket::close()
}
