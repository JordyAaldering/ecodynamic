mod corridor_controller;
mod delta_controller;

pub use corridor_controller::*;
pub use delta_controller::*;

pub trait Builder<Ctrl: Controller> {
    fn build(&self, max_threads: i32) -> Ctrl;
}

pub trait Controller {
    fn adjust_threads(&mut self, samples: Vec<f32>);

    fn num_threads(&self) -> i32;
}