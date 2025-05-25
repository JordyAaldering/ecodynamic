use clap::{Parser, Subcommand};
use controller::*;
use rapl_energy::Constraint;

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
    pub fn build(&self, req: Request) -> Box<dyn Controller> {
        use ControllerType::*;
        match &self.controller {
            Genetic(config) => {
                let max_power_uw = if let Some(constraint) = Constraint::now(0, 0, None) {
                    // Use current power limit if maximum is not defined
                    constraint.max_power_uw.unwrap_or(constraint.power_limit_uw)
                } else {
                    0
                };

                Box::new(GeneticController::new(req.max_threads, max_power_uw, self.letterbox_size, config.clone()))
            },
            Corridor(config) => Box::new(CorridorController::new(req.max_threads, config.clone())),
            Delta(config) => Box::new(DeltaController::new(req.max_threads as f32, config.clone())),
            Oscilating => Box::new(OscilatingController::new(req.max_threads)),
            Fixed => Box::new(FixedController::new(req.max_threads)),
        }
    }
}
