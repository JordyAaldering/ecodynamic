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

/// Median absolute deviation of the scores relative to their median, i.e. a measure of
/// how spread out this generation's scores are, independent of their absolute magnitude.
pub(crate) fn relative_score_spread(xs: &[f32]) -> f32 {
    let median_score = median(&mut xs.to_vec());
    if median_score.abs() < f32::EPSILON {
        return 0.0;
    }

    let mut relative_devs: Vec<f32> = xs.iter()
        .map(|&s| (s - median_score).abs() / median_score)
        .collect();
    median(&mut relative_devs)
}

/// Linearly interpolate between two values.
pub(crate) fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
