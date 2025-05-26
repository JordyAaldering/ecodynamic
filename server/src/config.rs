use clap::{Parser, Subcommand};
use controller::*;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,

    /// Size of the letterbox.
    #[arg(short('s'), long)]
    pub letterbox_size: usize,

    /// Run the resource controller for a single connection only.
    #[arg(long, action)]
    pub single: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ControllerType {
    /// Genetic algorithm approach.
    Genetic(GeneticControllerConfig),
    /// Algorithm based on a performance corridor.
    Corridor(CorridorControllerConfig),
    /// Algorithm based on deltas between runs.
    Delta(DeltaControllerConfig),
    /// Continuously oscilates between 1 and <max-threads>.
    Oscilating,
    /// Always returns <max-threads>.
    Fixed,
}

impl Config {
    pub fn build(&self, req: Request, power_limit_uw: u64) -> Box<dyn Controller> {
        use ControllerType::*;
        match &self.controller {
            Genetic(config) => {
                println!("Building genetic controller with {} max threads, {} max power, and config {:?}",
                         req.max_threads, power_limit_uw, config);
                Box::new(GeneticController::new(req.max_threads, power_limit_uw, self.letterbox_size, config.clone()))
            },
            Corridor(config) => Box::new(CorridorController::new(req.max_threads, config.clone())),
            Delta(config) => Box::new(DeltaController::new(req.max_threads as f32, config.clone())),
            Oscilating => Box::new(OscilatingController::new(req.max_threads)),
            Fixed => Box::new(FixedController::new(req.max_threads)),
        }
    }
}
