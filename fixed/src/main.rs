mod controller;

use std::{io::{self, BufReader}, os::unix::net::UnixStream, process, thread};

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
    let mut rdr = BufReader::new(stream.try_clone()?);

    // First message must be the application's capabilities
    let capabilities = socket::accept(&mut rdr)?;

    let mut tasks = ApplicationContext::new(
        1,
        || FixedController::new(capabilities.max_threads),
    );

    loop {
        match socket::read(&mut rdr)? {
            socket::Response::Request(request) => {
                let demand = tasks.request(request);
                socket::write(&mut stream, &demand)?;
            }
            socket::Response::Sample(sample) => {
                tasks.push(sample);
            }
            socket::Response::Disconnect => {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let Args {
        once,
    } = Args::parse();

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        socket::close().unwrap();
        process::exit(0);
    }).unwrap();

    if once {
        let stream = listener.incoming().next().unwrap()?;
        handle_client(stream)?;
    } else {
        for stream in listener.incoming().map_while(Result::ok) {
            thread::spawn(move || {
                handle_client(stream).unwrap()
            });
        }
    }

    socket::close()
}
