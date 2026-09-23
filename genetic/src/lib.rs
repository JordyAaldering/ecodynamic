mod capabilities;
mod controller;
mod chromosome;
mod gene;

pub use capabilities::HardwareCapabilities;
pub use controller::{GeneticController, GeneticConfig};
use chromosome::Chromosome;
use gene::*;
