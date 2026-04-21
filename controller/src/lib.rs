mod message;
mod controller;
mod filter_functions;

pub use message::*;
pub use controller::*;
use filter_functions::FilterFunction;

pub const LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

/// Compute the score for a set of samples, given an alpha parameter.
///
/// score = energy^alpha * runtime^(1 - alpha)
pub(crate) fn get_scores(samples: &[Sample], alpha: f32) -> Vec<f32> {
    samples.iter().map(|s| {
        get_score(s, alpha)
    }).collect()
}

/// Compute the score for a single sample, given an alpha parameter.
///
/// score = energy^alpha * runtime^(1 - alpha)
pub(crate) fn get_score(sample: &Sample, alpha: f32) -> f32 {
    sample.energy.powf(alpha) * sample.runtime.powf(1.0 - alpha)
}
