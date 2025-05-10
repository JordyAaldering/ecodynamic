use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use controller::{control::*, message::*};
use letterbox::{Letterbox, MTD_LETTERBOX_PATH};

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Controller type.
    #[arg(short('c'), long)]
    controller_type: ControllerType,

    /// Controller type.
    #[arg(short('f'), long)]
    score_function: ScoreFunction,

    /// Size of the letterbox.
    #[arg(short('s'), long)]
    letterbox_size: usize,

    /// Ignore samples with a score lower than <sample-cutoff>.
    #[arg(long, default_value_t = 0.0)]
    score_cutoff: f32,

    /// Genetic algorithm survival rate.
    #[arg(long, default_value_t = 0.50)]
    survival_rate: f32,

    /// Genetic algorithm mutation rate.
    #[arg(long, default_value_t = 0.25)]
    mutation_rate: f32,

    /// Genetic algorithm immigration rate.
    #[arg(long, default_value_t = 0.0)]
    immigration_rate: f32,

    /// Log received samples to this path.
    #[arg(long)]
    log_file: Option<PathBuf>,
}

#[derive(ValueEnum)]
#[derive(Copy, Clone, Debug)]
enum ControllerType {
    /// Genetic algorithm approach.
    Genetic,
    /// Algorithm based on a performance corridor.
    Corridor,
    /// Algorithm based on deltas between runs.
    Delta,
    /// Continuously oscilates between 1 and <max-threads>.
    Oscilating,
    /// Always returns <max-threads>.
    Fixed,
}

#[derive(ValueEnum)]
#[derive(Copy, Clone, Debug)]
enum ScoreFunction {
    Runtime,
    Energy,
}

impl ScoreFunction {
    fn score(self, sample: &Sample) -> f32 {
        use ScoreFunction::*;
        match self {
            Runtime => sample.runtime,
            Energy => sample.energy,
        }
    }
}

impl ControllerType {
    fn build(cli: &Cli, req: Request) -> Box<dyn Controller> {
        use ControllerType::*;
        match cli.controller_type {
            Genetic => {
                let config = GeneticControllerConfig {
                    population_size: cli.letterbox_size,
                    survival_rate: cli.survival_rate,
                    mutation_rate: cli.mutation_rate,
                    immigration_rate: cli.immigration_rate,
                };
                Box::new(GeneticController::new(req.max_threads, config))
            },
            Corridor => {
                Box::new(DeltaController::new(req.max_threads as f32))
            },
            Delta => {
                Box::new(CorridorController::new(req.max_threads))
            },
            Oscilating => {
                Box::new(OscilatingController::new(req.max_threads))
            },
            Fixed => {
                Box::new(FixedController::new(req.max_threads))
            },
        }
    }
}

fn handle_client(mut stream: UnixStream, cli: Cli) -> io::Result<()> {
    let mut letterbox = Letterbox::new(|req| ControllerType::build(&cli, req));

    let mut buffer = [0u8; Sample::SIZE];

    let mut log = if let Some(path) = &cli.log_file {
        println!("Creating log file at {:?}", path);
        let file = File::create_new(path)?;
        let mut w = BufWriter::new(file);
        w.write("uid,threads,runtime,usertime,energy\n".as_bytes())?;
        Some(w)
    } else {
        None
    };

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let req = Request::from(buf);
                debug_println!("Read: {:?}", req);

                // Update letterbox
                let demand = letterbox.try_get_demand(req);

                // Write to stream
                debug_println!("Send: {:?}", demand);
                let buf: [u8; Demand::SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(Sample::SIZE) => {
                let sample = Sample::from(buffer);

                debug_println!("Recv: {:?}", sample);

                if let Some(w) = &mut log {
                    w.write_fmt(format_args!("{},{},{},{},{}\n",
                        sample.region_uid,
                        letterbox.get_demand(sample.region_uid).num_threads,
                        sample.runtime,
                        sample.usertime,
                        sample.energy)
                    )?;
                }

                let score = cli.score_function.score(&sample);
                if score >= cli.score_cutoff {
                    letterbox.update(sample.region_uid, score);
                }
            }
            Ok(0) => {
                println!("Client disconnected");
                break;
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

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    let stream = listener.incoming().next().unwrap();
    match stream {
        Ok(stream) => handle_client(stream, args)?,
        Err(e) => eprintln!("Connection failed: {}", e),
    }

    println!("Server shutting down");
    fs::remove_file(MTD_LETTERBOX_PATH)?;
    Ok(())
}
