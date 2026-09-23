mod controller;

use std::{io::{self, BufReader}, os::unix::net::UnixStream, process, thread};

use clap::Parser;
use ecodynamic_core::*;

use crate::controller::DeltaController;

#[derive(Clone, Parser)]
struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    once: bool,
    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    idle_power: f32,
    /// Size of the letterbox for each task.
    #[arg(short('s'), long, default_value_t = 20)]
    letterbox_size: usize,
}

fn handle_client(mut stream: UnixStream, idle_power: f32, letterbox_size: usize) -> io::Result<()> {
    let mut rdr = BufReader::new(stream.try_clone()?);

    // First message must be the application's capabilities
    let capabilities = socket::accept(&mut rdr)?;

    let mut tasks = ApplicationContext::new(
        letterbox_size,
        || DeltaController::new(capabilities.max_threads),
    );

    loop {
        match socket::read(&mut rdr)? {
            socket::Response::Request(request) => {
                let demand = tasks.request(&request);
                socket::write(&mut stream, &demand)?;
            }
            socket::Response::Sample(mut sample) => {
                // Subtract idle energy
                sample.energy -= idle_power * sample.runtime;
                sample.energy = sample.energy.max(f32::EPSILON);
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
        idle_power,
        letterbox_size,
    } = Args::parse();

    let listener = socket::open()?;

    // Ensure the socket is closed when a control-C occurs
    ctrlc::set_handler(|| {
        socket::close().unwrap();
        process::exit(0);
    }).unwrap();

    if once {
        let stream = listener.incoming().next().unwrap()?;
        handle_client(stream, idle_power, letterbox_size)?;
    } else {
        for stream in listener.incoming().map_while(Result::ok) {
            thread::spawn(move || {
                handle_client(stream, idle_power, letterbox_size).unwrap()
            });
        }
    }

    socket::close()
}
