mod curves;

pub use curves::*;

use controller::{Sample, score};

fn median(xs: &[usize]) -> usize {
    let n = xs.len();
    if n % 2 == 0 {
        (xs[n / 2 - 1] + xs[n / 2]) / 2
    } else {
        xs[n / 2]
    }
}

pub fn quartiles(mut xs: Vec<usize>) -> (usize, usize, usize) {
    xs.sort_unstable();
    let n = xs.len();
    let med = median(&xs);
    let q1 = median(&xs[..n / 2]);
    let q3 = median(&xs[(n + 1) / 2..]);
    (med, q1, q3)
}

/// Estimates the globally best powercap in [power_min, power_max] for the current
/// synthetic runtime/energy curves and score definition.
///
/// The search is done by dense sampling: evaluate evenly spaced powercap values,
/// compute runtime and energy from the provided curves, transform those into the
/// controller score (energy^alpha * runtime^(1-alpha)), and keep the minimum.
///
/// This intentionally uses a noiseless baseline: runtime and energy are evaluated with
/// zero measurement noise so the result represents an ideal "perfect world" optimum.
/// The rest of the example then compares noisy controller samples against that reference
/// on purpose, to show how well the genetic algorithm approaches the best possible
/// configuration even when measurements are not perfect.
pub fn find_optimal_powercap(
	e_pref: f32,
    energy_curve: Curve,
	runtime_curve: Curve,
	power_min: f32,
	power_max: f32,
) -> (f32, f32, f32, f32) {
    let mut best_score = f32::INFINITY;
    let mut best_energy = f32::INFINITY;
    let mut best_runtime = f32::INFINITY;
    let mut best_powercap = power_min;

	for i in 0..=5000 {
		let t = i as f32 / 5000.0;
		let powercap = lerp(power_min, power_max, t);
        let energy=  energy_curve.eval(powercap, 0.0);
        let runtime = runtime_curve.eval(powercap, 0.0);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = score(&sample, e_pref);
		if score < best_score {
			best_score = score;
            best_energy = energy;
            best_runtime = runtime;
			best_powercap = powercap;
		}
	}

	(best_score, best_energy, best_runtime, best_powercap)
}

/// Derive a relative score-error threshold from the score definition and the
/// measurement noise model.
///
/// The score is computed as `energy^energy_preference * runtime^(1-energy_preference)`.
/// In this example, energy and runtime are sampled with normally distributed
/// multiplicative noise described by their coefficients of variation (CVs).
///
/// For modest noise levels, the relative variation of the combined score can be
/// approximated from the relative variations of its inputs. The energy term is
/// weighted by `energy_preference`, and the runtime term is weighted by
/// `1 - energy_preference`. Combining those two independent contributions in
/// quadrature gives an estimated relative score noise level.
///
/// We then multiply that estimated score noise by `threshold_multiplier` to get
/// the allowed relative score error for convergence checks. A multiplier of 2.0
/// means we accept scores that fall within roughly two derived standard
/// deviations of the noiseless optimum.
pub fn derive_score_error_threshold(
	e_pref: f32,
	energy_cv: f32,
	runtime_cv: f32,
	threshold_multiplier: f32,
) -> f32 {
	let runtime_preference = 1.0 - e_pref;
	let derived_score_cv = f32::sqrt(
        (e_pref * energy_cv).powi(2)
        + (runtime_preference * runtime_cv).powi(2)
    );
	derived_score_cv * threshold_multiplier
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
