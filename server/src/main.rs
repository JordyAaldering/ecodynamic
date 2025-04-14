use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, Read, Write};
use std::sync::Arc;
use std::{fs, mem};

use controller::*;
use letterbox::*;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Controller type
    #[arg(short, long)]
    controller_type: ControllerType,

    /// Size of the letterbox
    #[arg(short('s'), long)]
    letterbox_size: usize,

    /// Genetic algorithm survival rate
    #[arg(long, default_value_t = 0.50)]
    survival_rate: f32,

    /// Genetic algorithm mutation rate
    #[arg(long, default_value_t = 0.25)]
    mutation_rate: f32,
}

#[derive(ValueEnum)]
#[derive(Copy, Clone, Debug)]
enum ControllerType {
    /// Genetic algorithm approach
    GeneticAlgorithm,
    /// Algorithm based on a performance corridor
    CorridorBased,
    /// Algorithm based on deltas between runs
    DeltaBased,
}

fn build_controller(cli: Arc<Cli>, req: Request) -> Box<dyn Controller> {
    match cli.controller_type {
        ControllerType::GeneticAlgorithm => Box::new(GeneticController::new(req.max_threads, cli.letterbox_size, cli.survival_rate, cli.mutation_rate)),
        ControllerType::CorridorBased => Box::new(DeltaController::new(req.max_threads)),
        ControllerType::DeltaBased => Box::new(CorridorController::new(req.max_threads)),
    }
}

fn handle_client(mut stream: UnixStream, cli: Arc<Cli>) -> std::io::Result<()> {
    let mut letterbox = Letterbox::new(|req| build_controller(cli.clone(), req));

    const READREQ_SIZE: usize = mem::size_of::<Request>();
    const SAMPLE_SIZE: usize = mem::size_of::<Sample>();
    const DEMAND_SIZE: usize = mem::size_of::<Demand>();
    let mut buffer = [0u8; SAMPLE_SIZE];

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(READREQ_SIZE) => {
                let buf: [u8; READREQ_SIZE] = buffer[0..READREQ_SIZE].try_into().unwrap();
                let req = Request::from(buf);
                println!("Read: {:?}", req);

                // Update letterbox
                let demand = letterbox.read(req);

                // Write to stream
                println!("Send: {:?}", demand);
                let buf: [u8; DEMAND_SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(SAMPLE_SIZE) => {
                let sample = Sample::from(buffer);
                println!("Recv: {:?}", sample);
                letterbox.update(sample);
            }
            Ok(n) => {
                eprintln!("Invalid message size: {}", n);
                break;
            }
            Err(e) => {
                eprintln!("Client disconnected: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let args = Arc::new(args);

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cli = args.clone();
                std::thread::spawn(move || {
                    handle_client(stream, cli)
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    unreachable!()
}
