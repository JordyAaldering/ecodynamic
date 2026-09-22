mod controller;

use std::{
    collections::HashMap,
    io::{self, BufReader},
    os::unix::net::UnixStream,
    process,
    thread,
};

use clap::Parser;
use ecodynamic_api::*;
use ecodynamic_core::*;

use crate::controller::FixedController;

#[derive(Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,
}

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    let mut lbs: HashMap<i32, FixedController> = HashMap::new();
    let mut rdr = BufReader::new(stream.try_clone()?);

    // First message must be the application's capabilities
    let capabilities = socket::accept(&mut rdr)?;

    loop {
        match socket::read(&mut rdr)? {
            socket::Response::Request(request) => {
                let controller = lbs.entry(request.region_uid)
                    .or_insert_with(|| {
                        log::debug!("Generating controller for request {}", request.region_uid);
                        FixedController::new(&capabilities)
                    });

                let demand = controller.get_demand();
                socket::write(&mut stream, &demand)?;
            }
            socket::Response::Sample(sample) => {
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
        handle_client(stream)?
    } else {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream).unwrap()
                    });
                }
                Err(e) => log::error!("Connection failed: {}", e),
            }
        }
    }

    socket::close()
}
