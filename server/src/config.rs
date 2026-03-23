use clap::{Parser, Subcommand};
use controller::*;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// Exit after handling a single client.
    #[arg(long, action)]
    pub once: bool,

    /// Idle power draw of the processor.
    #[arg(short('w'), long("idle"), default_value_t = 0.0)]
    pub idle_power: f32,

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
    pub fn build_controller(&self) -> Box<dyn Controller> {
        use ControllerType::*;
        match &self.controller {
            Genetic(config) => Box::new(GeneticController::new(config.clone())),
            Corridor(config) => Box::new(CorridorController::new(config.clone())),
            Delta(config) => Box::new(DeltaController::new(config.clone())),
            Oscilating => Box::new(OscilatingController::new()),
            Fixed => Box::new(FixedController::new()),
        }
    }
}
