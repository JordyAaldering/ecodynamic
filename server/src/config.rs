use std::{path::PathBuf, sync::{Arc, Mutex}};

use clap::{Parser, ValueEnum};
use controller::*;
use letterbox::*;

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

    /// Ignore samples with a score lower than <sample-cutoff>.
    #[arg(long, default_value_t = 0.0)]
    pub score_cutoff: f32,

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
}

impl GeneticControllerConfig for Config {
    fn population_size(&self) -> usize {
        self.letterbox_size
    }

    fn survival_rate(&self) -> f32 {
        self.survival_rate
    }

    fn mutation_rate(&self) -> f32 {
        self.mutation_rate
    }

    fn immigration_rate(&self) -> f32 {
        self.immigration_rate
    }
}

#[derive(ValueEnum)]
#[derive(Copy, Clone, Debug)]
pub enum ControllerType {
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

impl ControllerType {
    pub fn build(config: Arc<Mutex<Config>>, req: Request) -> Box<dyn Controller> {
        use ControllerType::*;
        match config.lock().unwrap().controller_type {
            Genetic => {
                Box::new(GeneticController::new(req.max_threads, config.clone()))
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

#[derive(ValueEnum)]
#[derive(Copy, Clone, Debug)]
pub enum ScoreFunction {
    Runtime,
    Energy,
}

impl ScoreFunction {
    pub fn score(self, sample: &Sample) -> f32 {
        use ScoreFunction::*;
        match self {
            Runtime => sample.runtime,
            Energy => sample.energy,
        }
    }
}
