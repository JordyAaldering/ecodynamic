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

use crate::{Demand, Sample};

pub trait Controller {
    fn get_demand(&mut self) -> Demand;

    fn push(&mut self, sample: Sample);
}
