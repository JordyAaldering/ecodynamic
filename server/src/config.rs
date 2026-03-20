use clap::{Parser, Subcommand};
use controller::*;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// Size of the letterbox.
    #[arg(short('s'), long, default_value_t = 20)]
    pub letterbox_size: usize,

    /// Idle power draw of the processor.
    #[arg(short('w'), long, default_value_t = 0.0)]
    pub idle_power: f32,

    /// If a specific command is provided, run the resource controller for that process only.
    ///
    /// This must be a single command, without arguments, due to the way argument parsing works.
    #[arg(short('c'), long)]
    pub cmd: Option<String>,

    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,
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

impl Args {
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
