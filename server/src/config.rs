mod score_fn;

use clap::{Parser, Subcommand};
use controller::*;
pub use score_fn::*;

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::CONFIG;

#[derive(Clone)]
pub struct SharedConfig {
    inner: Arc<Mutex<Config>>,
}

impl SharedConfig {
    pub fn new(config: Config) -> Self {
        Self {
            inner: Arc::new(Mutex::new(config)),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, Config> {
        self.inner.lock().unwrap()
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,

    /// Controller type.
    #[arg(short('f'), long)]
    pub score_function: ScoreFunction,

    /// Size of the letterbox.
    #[arg(short('s'), long)]
    pub letterbox_size: usize,

    /// Log received samples to this path.
    /// Creates a file for each client.
    #[arg(long)]
    pub log_path: Option<PathBuf>,

    /// Run the resource controller for a single connection only
    #[arg(long, action)]
    pub single: bool,
}

#[derive(Debug)]
#[derive(Subcommand)]
pub enum ControllerType {
    /// Genetic algorithm approach.
    Genetic(GeneticControllerConfig),
    /// Algorithm based on a performance corridor.
    Corridor,
    /// Algorithm based on deltas between runs.
    Delta,
    /// Continuously oscilates between 1 and <max-threads>.
    Oscilating,
    /// Always returns <max-threads>.
    Fixed,
}

impl ControllerType {
    pub fn build(req: Request) -> Box<dyn Controller> {
        use ControllerType::*;
        match &CONFIG.lock().controller {
            Genetic(config) => Box::new(GeneticController::new(req.max_threads, CONFIG.lock().letterbox_size, config.clone())),
            Corridor   => Box::new(DeltaController::new(req.max_threads as f32)),
            Delta      => Box::new(CorridorController::new(req.max_threads)),
            Oscilating => Box::new(OscilatingController::new(req.max_threads)),
            Fixed      => Box::new(FixedController::new(req.max_threads)),
        }
    }
}
