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

use crate::{Demand, Sample, State};

pub trait Controller {
    /// Get the current configuration recommendation.
    fn get_demand(&self) -> Demand;

    /// Remember the current state of the system.
    fn store_state(&mut self, _state: State) { }

    /// Push the results of the previous configuration.
    fn push_sample(&mut self, sample: Sample);
}
