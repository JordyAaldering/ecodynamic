mod corridor_controller;
mod delta_controller;
mod genetic_controller;

pub use corridor_controller::*;
pub use delta_controller::*;
pub use genetic_controller::*;

use crate::{Demand, Sample};

pub trait Controller {
    fn sample_received(&mut self, sample: Sample);

    fn next_demand(&mut self) -> Demand;
}
