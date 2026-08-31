mod pinning_strategy;
mod powercap;
mod thread_count;

pub use pinning_strategy::PinningStrategy;
pub use powercap::Powercap;
pub use thread_count::ThreadCount;

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
