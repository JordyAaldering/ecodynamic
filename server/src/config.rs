mod controller_type;
mod score_fn;

use clap::Parser;
pub use controller_type::*;
pub use score_fn::*;

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

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
    #[arg(short('c'), long)]
    pub controller_type: ControllerType,

    /// Controller type.
    #[arg(short('f'), long)]
    pub score_function: ScoreFunction,

    /// Size of the letterbox.
    #[arg(short('s'), long)]
    pub letterbox_size: usize,

    /// Genetic algorithm survival rate.
    #[arg(long, default_value_t = 0.50)]
    pub survival_rate: f32,

    /// Genetic algorithm mutation rate.
    #[arg(long, default_value_t = 0.25)]
    pub mutation_rate: f32,

    /// Genetic algorithm immigration rate.
    #[arg(long, default_value_t = 0.0)]
    pub immigration_rate: f32,

    /// Log received samples to this path.
    /// Creates a file for each client.
    #[arg(long)]
    pub log_path: Option<PathBuf>,

    /// Run the resource controller for a single connection only
    #[arg(long, action)]
    pub single: bool
}
