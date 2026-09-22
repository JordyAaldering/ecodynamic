mod controller;

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    os::unix::net::UnixStream,
    process,
    thread,
};

use clap::Parser;
use ecodynamic_api::*;
use ecodynamic_core::socket;

use crate::controller::{CorridorController, Config};

#[derive(Clone, Debug, Parser)]
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
    let mut line = String::new();

    // First message must be a capabilities broadcast from the client
    rdr.read_line(&mut line)?;
    let capabilities: AppCapabilities = serde_json::from_str(line.trim_end())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Expected capabilities: {e}")))?;
    log::debug!("Client capabilities: {capabilities:?}");

    loop {
        match socket::response(&mut rdr)? {
            socket::Response::Request(request) => {
                let controller = lbs.entry(request.region_uid)
                    .or_insert_with(|| {
                        log::info!("Generating controller for request {}", request.region_uid);
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
                    .expect("Received sample for region that has not yet been instantiated")
                    .push_sample(sample);
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
    log::trace!("Args: {args:?}");

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        socket::close().unwrap();
        process::exit(0);
    }).unwrap();

    if args.once {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, args)?,
            Err(e) => log::error!("Connection failed: {}", e),
        }
    } else {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let args = args.clone();
                    thread::spawn(move || {
                        handle_client(stream, args).unwrap()
                    });
                }
                Err(e) => log::error!("Connection failed: {}", e),
            }
        }
    }

    socket::close()
}
