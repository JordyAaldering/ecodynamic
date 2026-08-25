mod controller;
mod direction;
mod filter_functions;
mod message;

use std::sync::atomic::AtomicU16;

pub use controller::*;
pub(crate) use direction::*;
pub(crate) use filter_functions::*;
pub use message::*;

pub const LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

/// Track the total number of threads currently in use across all clients.
/// This can be used to steer configurations towards efficiently sharing available resources.
pub static GLOBAL_THREAD_COUNT: AtomicU16 = AtomicU16::new(0);

/// Temporary value, until I convert this to a proper configuration parameter.
pub(crate) const AVAILABLE_CORES: u16 = 16;

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
