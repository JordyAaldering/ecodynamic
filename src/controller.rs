mod controller_energy;
mod controller_runtime;
mod controller_fixed;
mod direction;

pub use controller_energy::EnergyController;
pub use controller_runtime::RuntimeController;
pub use controller_fixed::FixedController;

pub trait Controller {
    fn adjust_threads(&mut self, tn: f32) -> f32;
}
