mod energy;
mod runtime;

pub use energy::SampleEnergy;
pub use runtime::SampleRuntime;

pub trait Sample {
    fn start(&mut self);

    fn stop(&self) -> f32;
}
