use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, Read, Write};
use std::sync::Arc;
use std::fs;

use clap::{Parser, ValueEnum};

use controller::*;
use letterbox::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Controller type
    #[arg(short('c'), long)]
    controller_type: ControllerType,

    /// Controller type
    #[arg(short('f'), long)]
    score_function: ScoreFunction,

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

#[derive(ValueEnum)]
#[derive(Copy, Clone, Debug)]
enum ScoreFunction {
    Runtime,
    Energy,
}

fn build_controller(cli: Arc<Cli>, req: Request) -> Box<dyn Controller> {
    let score_fn: fn(Sample) -> f32 = match cli.score_function {
        ScoreFunction::Runtime => |s| s.runtime,
        ScoreFunction::Energy => |s| s.energy,
    };

    match cli.controller_type {
        ControllerType::GeneticAlgorithm => {
            let settings = GeneticControllerSettings {
                score_fn,
                max_threads: req.max_threads,
                population_size: cli.letterbox_size,
                survival_rate: cli.survival_rate,
                mutation_rate: cli.mutation_rate,
            };
            Box::new(GeneticController::new(settings))
        },
        ControllerType::CorridorBased => {
            let settings = DeltaControllerSettings {
                score_fn,
                max_threads: req.max_threads,
                population_size: cli.letterbox_size,
            };
            Box::new(DeltaController::new(settings))
        },
        ControllerType::DeltaBased => {
            let settings = CorridorControllerSettings {
                score_fn,
                max_threads: req.max_threads,
                population_size: cli.letterbox_size,
            };
            Box::new(CorridorController::new(settings))
        },
    }
}

fn handle_client(mut stream: UnixStream, cli: Arc<Cli>) -> io::Result<()> {
    let mut letterbox = Letterbox::new(|req| build_controller(cli.clone(), req));

    let mut buffer = [0u8; Sample::SIZE];

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let req = Request::from(buf);
                println!("Read: {:?}", req);

                // Update letterbox
                let demand = letterbox.read(req);

                // Write to stream
                println!("Send: {:?}", demand);
                let buf: [u8; Demand::SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(Sample::SIZE) => {
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
