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

use crate::Sample;

pub trait Controller {
    fn evolve(&mut self, samples: Vec<Sample>);

    fn num_threads(&mut self) -> i32;

    fn power_limit_uw(&mut self) -> u64;
}
