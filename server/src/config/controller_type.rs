use controller::*;

use crate::CONFIG;

use super::SharedConfig;

#[derive(clap::ValueEnum)]
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

impl GeneticControllerConfig for SharedConfig {
    fn population_size(&self) -> usize {
        self.lock().letterbox_size
    }

    fn survival_count(&self) -> usize {
        (self.lock().letterbox_size as f32 * self.lock().survival_rate).round() as usize
    }

    fn immigration_count(&self) -> usize {
        (self.lock().letterbox_size as f32 * self.lock().immigration_rate).round() as usize
    }

    fn mutation_rate(&self) -> f32 {
        self.lock().mutation_rate
    }
}

impl ControllerType {
    pub fn build(req: Request) -> Box<dyn Controller> {
        use ControllerType::*;
        match CONFIG.lock().controller_type {
            Genetic    => Box::new(GeneticController::new(req.max_threads, CONFIG.clone())),
            Corridor   => Box::new(DeltaController::new(req.max_threads as f32)),
            Delta      => Box::new(CorridorController::new(req.max_threads)),
            Oscilating => Box::new(OscilatingController::new(req.max_threads)),
            Fixed      => Box::new(FixedController::new(req.max_threads)),
        }
    }
}
