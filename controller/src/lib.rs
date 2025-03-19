mod corridor_controller;
mod delta_controller;
mod direction;
mod percentage;

pub use corridor_controller::*;
pub use delta_controller::*;

pub trait Controller : Default {
    fn adjust_threads(&mut self, samples: Vec<f32>);

    fn num_threads(&self) -> u8;
}
