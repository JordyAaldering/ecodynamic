use clap::{Parser, Subcommand};
use controller::*;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,

    /// Size of the letterbox.
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,

    /// Idle power draw of the system. Leave empty to determine automatically, set to 0 to
    /// ignore idle power, or set to a specific value if idle power is predetermined.
    #[arg(short('w'), long)]
    pub idle_power: Option<f32>,

    /// Run the resource controller for a single connection only.
    #[arg(long, action)]
    pub once: bool,
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
    pub fn build(&self) -> Box<dyn Controller> {
        use ControllerType::*;
        match &self.controller {
            Genetic(config) => Box::new(GeneticController::new(self.letterbox_size, config.clone())),
            Corridor(config) => Box::new(CorridorController::new(config.clone())),
            Delta(config) => Box::new(DeltaController::new(config.clone())),
            Oscilating => Box::new(OscilatingController::new()),
            Fixed => Box::new(FixedController::new()),
        }
    }
}
