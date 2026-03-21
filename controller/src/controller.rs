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
    /// Gets the demand (e.g. power limit or thread count) for the current runtime conditions.
    ///
    /// Note that for the very first iteration, this will be called before any samples have been collected.
    fn get_demand(&self) -> (GlobalDemand, LocalDemand);

    /// A new sample was received.
    ///
    /// Either this directly results in an 'evolution' of the controller, or the sample is
    /// stored until enough samples have been collected to perform an evolution step.
    fn push_sample(&mut self, sample: Sample);
}
