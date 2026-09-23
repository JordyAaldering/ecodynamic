#![allow(unused)]

use std::{io, str::FromStr};

use ecodynamic_api::Sample;
use ecodynamic_core::lerp;
use rand::distr::Distribution;
use rand_distr::Normal;

#[derive(Clone, Copy, Debug)]
pub enum Curve {
	/// ```tex
	/// f(\x) = lb + (ub - lb) * \x};
	/// ```
	Linear {
		lb: f32,
		ub: f32,
	},
	/// ```tex
	/// f(\x) = lb + steepness * (\x - t_middle)^2;
	/// ```
	Quadratic {
        lb: f32,
		t_middle: f32,
        steepness: f32,
	},
	/// ```tex
	/// f(\x) = lb + (ub - lb) * 0.5 * (1 + tanh((\x - t_middle) * steepness));
	/// ```
	Sigmoid {
		lb: f32,
		ub: f32,
		t_middle: f32,
		steepness: f32,
	},
}

impl FromStr for Curve {
	type Err = io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let (variant, values) = input.split_once(':').unwrap();
		let mut values = values
			.split(',')
			.map(|s| s.parse::<f32>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, s)));

		Ok(match variant {
			"Linear" => Self::Linear {
				lb: values.next().unwrap()?,
				ub: values.next().unwrap()?,
			},
			"Quadratic" => Self::Quadratic {
				lb: values.next().unwrap()?,
				t_middle: values.next().unwrap()?,
				steepness: values.next().unwrap()?,
			},
			"Sigmoid" => Self::Sigmoid {
				lb: values.next().unwrap()?,
				ub: values.next().unwrap()?,
				t_middle: values.next().unwrap()?,
				steepness: values.next().unwrap()?,
			},
			_ => return Err(io::Error::new(io::ErrorKind::InvalidInput, variant)),
		})
	}
}

impl ToString for Curve {
	fn to_string(&self) -> String {
		match self {
			Self::Linear { lb, ub } =>
				format!("Linear:{lb:.2},{ub:.2}"),
			Self::Quadratic { lb, t_middle, steepness } =>
				format!("Quadratic:{lb:.2},{t_middle:.2},{steepness:.2}"),
			Self::Sigmoid { lb, ub, t_middle, steepness } =>
				format!("Sigmoid:{lb:.2},{ub:.2},{t_middle:.2},{steepness:.2}"),
		}
	}
}

impl Curve {
	pub fn eval(&self, t: f32, cv: f32) -> f32 {
        assert!(t >= 0.0);
		assert!(t <= 1.0);
        assert!(cv >= 0.0);
		let v = match self {
			Self::Linear { lb, ub } => {
				lb + (ub - lb) * t
			}
			Self::Quadratic { lb, t_middle, steepness } => {
                let t_dt = t - t_middle;
                lb + steepness * (t_dt).powi(2)
			}
			Self::Sigmoid { lb, ub, t_middle, steepness } => {
                let v_dt = ub - lb;
                let t_dt = t - t_middle;
                lb + v_dt * (0.5 * (1.0 + f32::tanh(t_dt * steepness)))
			}
		};
        sample_normal_value(v.max(0.01), cv).max(f32::EPSILON)
	}
}

fn sample_normal_value(mean: f32, cv: f32) -> f32 {
	if cv <= 0.0 {
		return mean;
	}
	let std = mean * cv;
	let mut rng = rand::rng();
	let normal = Normal::new(mean, std).unwrap();
	normal.sample(&mut rng)
}

pub fn quartiles(mut xs: Vec<usize>) -> (usize, usize, usize) {
    fn median(xs: &[usize]) -> usize {
        let n = xs.len();
        if n % 2 == 0 {
            (xs[n / 2 - 1] + xs[n / 2]) / 2
        } else {
            xs[n / 2]
        }
    }

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

    const SAMPLES: usize = 5000;
	for i in 0..=SAMPLES {
		let t = i as f32 / SAMPLES as f32;
		let powercap = lerp(power_min, power_max, t);
        let energy=  energy_curve.eval(powercap, 0.0);
        let runtime = runtime_curve.eval(powercap, 0.0);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = sample.score(e_pref);
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
    let e_noise = (e_pref * energy_cv).powi(2);
    let r_noise = ((1.0 - e_pref) * runtime_cv).powi(2);
	threshold_multiplier * f32::sqrt(e_noise + r_noise)
}
