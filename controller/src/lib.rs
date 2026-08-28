mod controller;
mod direction;
mod filter_functions;
mod message;
mod state;

pub use controller::*;
pub(crate) use direction::*;
pub(crate) use filter_functions::*;
pub use message::*;
pub use state::*;

pub const LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

/// Compute the score for a set of samples, given an alpha parameter.
///
/// score = energy^alpha * runtime^(1 - alpha)
pub(crate) fn score(samples: &[Sample], alpha: f32) -> Vec<f32> {
    samples.iter().map(|sample| {
        sample.score(alpha)
    }).collect()
}
