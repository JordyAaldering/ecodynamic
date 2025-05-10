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

pub trait Controller {
    fn evolve(&mut self, scores: Vec<f32>);

    fn next_demand(&mut self) -> i32;
}
