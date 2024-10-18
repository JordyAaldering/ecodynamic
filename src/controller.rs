mod direction;
mod selection;
mod thread_count;

pub use direction::Direction;
pub use selection::{SelectionAlgorithm, FrequencyDist};
pub use thread_count::ThreadCount;

pub trait Controller {
    fn adjust_threads(&mut self, tn: f32) -> f32;
}
