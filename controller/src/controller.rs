mod genetic;

pub use genetic::*;

use crate::{Demand, Sample, State};

pub trait Controller {
    /// Get the current configuration recommendation.
    fn get_demand(&self) -> Demand;

    /// Remember the current state of the system.
    fn store_state(&mut self, _state: State) { }

    /// Push the results of the previous configuration.
    fn push_sample(&mut self, sample: Sample);
}
