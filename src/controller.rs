mod direction;
mod thread_count;
mod controller_energy;
mod controller_runtime;
mod controller_fixed;

pub use controller_energy::EnergyController;
pub use controller_runtime::RuntimeController;
pub use controller_fixed::FixedController;

pub trait Controller {
    fn adjust_threads(&mut self, tn: f32) -> f32;
}
