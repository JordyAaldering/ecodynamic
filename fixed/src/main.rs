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
use ecodynamic_core::*;

use crate::controller::FixedController;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,
    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    pub idle_power: f32,
}

fn handle_client(mut stream: UnixStream, args: Args) -> io::Result<()> {
    let mut lbs: HashMap<i32, FixedController> = HashMap::new();
    let mut rdr = BufReader::new(stream.try_clone()?);
    let mut line = String::new();

    // First message must be a capabilities broadcast from the client
    rdr.read_line(&mut line)?;
    let capabilities: AppCapabilities = serde_json::from_str(line.trim_end())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Expected capabilities: {e}")))?;
    log::debug!("Client capabilities: {capabilities:?}");

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
                    sample.energy -= args.idle_power * sample.runtime;
                    sample.energy = sample.energy.max(f32::EPSILON);

                    lbs.get_mut(&sample.region_uid)
                        .expect("Received sample for region that has not yet been instantiated")
                        .push_sample(sample);
                } else if let Ok(request) = serde_json::from_str::<Request>(&line) {
                    log::trace!("GET: {:?}", request.region_uid);

                    let controller = lbs.entry(request.region_uid)
                        .or_insert_with(|| {
                            log::info!("Generating controller for request {}", request.region_uid);
                            FixedController::new(&capabilities)
                        });

                    let demand = controller.get_demand();
                    socket::write(&mut stream, &demand)?;
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
        let stream = listener.incoming().next().unwrap()?;
        handle_client(stream, args)?
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
