mod controller;

use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader, Write},
    os::unix::{fs::PermissionsExt, net::{UnixListener, UnixStream}},
    process,
    thread,
};

use clap::Parser;
use ecodynamic_api::*;
use ecodynamic_core::*;

use crate::controller::{DeltaController, Config};

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
    let mut lbs: HashMap<i32, DeltaController> = HashMap::new();
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
                            DeltaController::new(&args.config, &capabilities)
                        });

                    let demand = controller.get_demand();
                    log::trace!("PUT: {:?}", demand);

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

fn main() {
    env_logger::init();

    let args = Args::parse();
    log::trace!("Args: {args:?}");

    let listener = open_socket();

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        close_socket();
        process::exit(0);
    }).unwrap();

    if args.once {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, args).unwrap(),
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

    close_socket();
}

fn open_socket() -> UnixListener {
    if fs::metadata(LETTERBOX_PATH).is_ok() {
        log::warn!("Closing previous socket: {}", LETTERBOX_PATH);
        fs::remove_file(LETTERBOX_PATH).expect("Could not close socket");
    }
    log::info!("Creating socket: {}", LETTERBOX_PATH);
    let listener = UnixListener::bind(LETTERBOX_PATH)
        .expect("Could not create socket");
    fs::set_permissions(LETTERBOX_PATH, fs::Permissions::from_mode(0o666))
        .expect("Failed to set socket permissions");
    listener
}

fn close_socket() {
    log::info!("Closing socket: {}", LETTERBOX_PATH);
    fs::remove_file(LETTERBOX_PATH).expect("Could not close socket");
}
