mod corridor_controller;
mod delta_controller;
mod genetic_controller;
mod demand;
mod sample;

pub use corridor_controller::*;
pub use delta_controller::*;
pub use genetic_controller::*;
pub use demand::Demand;
pub use sample::Sample;

pub trait Builder<Ctrl: Controller> {
    fn build(&self, max_threads: i32) -> Ctrl;
}

pub trait Controller {
    fn adjust_threads(&mut self, samples: Vec<f32>);

    fn num_threads(&self) -> i32;
}
