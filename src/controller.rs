mod direction;
mod selection;
mod thread_count;

pub use direction::Direction;
pub use selection::{SelectionAlgorithm, FrequencyDist};
pub use thread_count::ThreadCount;

use crate::Sample;

pub trait Controller {
    fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32;
}
