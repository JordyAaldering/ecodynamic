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

use crate::message::Demand;

pub trait Controller {
    fn sample_received(&mut self, score: f32);

    fn evolve(&mut self, scores: Vec<f32>);

    fn get_demand(&self) -> Demand;
}
