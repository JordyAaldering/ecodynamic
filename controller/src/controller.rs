mod corridor;
mod delta;
mod fixed;
mod genetic;
mod oscilating;

pub use corridor::*;
pub use delta::*;
pub use fixed::*;
pub use genetic::*;
pub use oscilating::*;

use crate::{GlobalDemand, LocalDemand, Sample};

pub trait Controller {
    fn evolve(&mut self, samples: Vec<Sample>);

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand);
}
