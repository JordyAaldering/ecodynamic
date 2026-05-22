use std::{io, str::FromStr};

use clap::Parser;
use rand::distr::Distribution;
use rand_distr::Normal;

use controller::*;

const MAX_ITERATIONS: usize = 200;
/// We consider the controller converged once enough recent iterations stay close to the
/// predicted best score: at least CONVERGENCE_REQUIRED of the last CONVERGENCE_WINDOW
/// iterations must be within a derived score-error threshold of that best score.
const CONVERGENCE_WINDOW: usize = 20;
const CONVERGENCE_REQUIRED: usize = 15;
const CONVERGENCE_THRESHOLD_MULTIPLIER: f32 = 1.5;

#[derive(Clone, Debug, Parser)]
pub struct Args {
	/// Coefficient of variation for energy measurements.
    ///
    /// Example, if the CV is 0.05 (5%) and the expected energy is 100J, then
    /// for a normal distribution about 68% of measurements will be in the
    /// range [95J, 105J], and about 95% will be in the range [90J, 110J].
    #[arg(long, default_value_t = 0.025)]
    energy_cv: f32,

	/// Coefficient of variation for runtime measurements.
    #[arg(long, default_value_t = 0.005)]
    runtime_cv: f32,

	/// Energy curve in the form `Linear:min,max`, `Quadratic:t_optimum,value_at_optimum`,
	/// or `Sigmoid:min,max,t_middle,steepness`.
	///
	/// Reasonable defaults for the current example are `Linear:60,90`,
	/// `Quadratic:0.75,60`, and `Sigmoid:20,90,0.5,0.8`.
	#[arg(long, default_value = "Quadratic:0.75,60.0")]
    energy_curve: EnergyCurve,

	/// Runtime curve in the form `Linear:min,max`, `Quadratic:t_optimum,value_at_optimum`,
	/// or `Sigmoid:min,max,t_middle,steepness`.
	#[arg(long, default_value = "Sigmoid:50.0,20.0,0.5,0.8")]
    runtime_curve: RuntimeCurve,

    #[command(flatten)]
    config: GeneticControllerConfig,
}

/// Curve families used to synthesize energy measurements.
///
/// Example TikZ scaffold:
/// ```tex
/// \begin{tikzpicture}
/// \begin{axis}[
///   declare function={
///     energy_lb = 60;
///     energy_ub = 90;
///     f(\x) = energy_lb + (energy_ub - energy_lb) * \x;
///     runtime_lb = 60;
///     runtime_ub = 90;
///     t_optimum = 0.5;
///     g(\x) = runtime_lb * (1 + ((\x - t_optimum) / max(t_optimum, 1 - t_optimum))^2);
///     alpha = 0.5;
///     score(\x) = f(\x)^alpha * g(\x)^(1 - alpha);
///   },
/// ]
///   \addplot[domain=0:1,samples=100,color=green] {f(x)};
///   \addplot[domain=0:1,samples=100,color=red] {g(x)};
///   \addplot[domain=0:1,samples=100,color=cyan] {score(x)};
/// \end{axis}
/// \end{tikzpicture}
/// ```
#[derive(Clone, Copy, Debug)]
enum EnergyCurve {
	/// ```tex
	/// f(\x) = energy_at_min_power + (energy_at_max_power - energy_at_min_power) * \x
	/// ```
	/// Reasonable example default: `Linear:60,90`.
	Linear {
		energy_at_min_power: f32,
		energy_at_max_power: f32,
	},
	/// ```tex
	/// f(\x) = energy_at_optimum * (1 + ((\x - t_optimum) / max(t_optimum, 1 - t_optimum))^2)
	/// ```
	/// Reasonable example default: `Quadratic:0.75,60`.
	Quadratic {
		t_optimum: f32,
		energy_at_optimum: f32,
	},
	/// ```tex
	/// curve_strength = 2 + 10 * steepness;
	/// f(\x) = energy_at_min + (energy_at_max - energy_at_min) * 0.5 * (1 + tanh((\x - t_middle) * curve_strength))
	/// ```
	/// Reasonable example default: `Sigmoid:20,90,0.5,0.8`.
	Sigmoid {
		min_energy: f32,
		max_energy: f32,
		t_middle: f32,
		steepness: f32,
	},
}

#[derive(Clone, Copy, Debug)]
enum RuntimeCurve {
	Linear {
		runtime_at_min_power: f32,
		runtime_at_max_power: f32,
	},
	Quadratic {
		t_optimum: f32,
		runtime_at_optimum: f32,
	},
	Sigmoid {
		min_runtime: f32,
		max_runtime: f32,
		t_middle: f32,
		steepness: f32,
	},
}

impl FromStr for EnergyCurve {
	type Err = io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let (variant, values) = input.split_once(':').unwrap();
		let values = values
			.split(',')
			.map(str::parse::<f32>)
			.collect::<Result<Vec<_>, _>>()
			.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, input.to_string()))?;

		Ok(match variant {
			"Linear" => EnergyCurve::Linear {
				energy_at_min_power: values[0],
				energy_at_max_power: values[1],
			},
			"Quadratic" => EnergyCurve::Quadratic {
				t_optimum: values[0],
				energy_at_optimum: values[1],
			},
			"Sigmoid" => {
				EnergyCurve::Sigmoid {
					min_energy: values[0],
					max_energy: values[1],
					t_middle: values[2],
					steepness: values[3],
				}
			}
			_ => return Err(io::Error::new(io::ErrorKind::InvalidInput, input.to_string())),
		})
	}
}

impl FromStr for RuntimeCurve {
	type Err = io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let (variant, values) = input.split_once(':').unwrap();
		let values = values
			.split(',')
			.map(str::parse::<f32>)
			.collect::<Result<Vec<_>, _>>()
			.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, input.to_string()))?;

		Ok(match variant {
			"Linear" => RuntimeCurve::Linear {
				runtime_at_min_power: values[0],
				runtime_at_max_power: values[1],
			},
			"Quadratic" => RuntimeCurve::Quadratic {
				t_optimum: values[0],
				runtime_at_optimum: values[1],
			},
			"Sigmoid" => {
				RuntimeCurve::Sigmoid {
					min_runtime: values[0],
					max_runtime: values[1],
					t_middle: values[2],
					steepness: values[3],
				}
			}
			_ => return Err(io::Error::new(io::ErrorKind::InvalidInput, input.to_string())),
		})
	}
}

impl EnergyCurve {
	fn eval(self, t: f32, cv: f32) -> f32 {
        debug_assert!(t >= 0.0 && t <= 1.0);
        use EnergyCurve::*;
		match self {
			Linear { energy_at_min_power, energy_at_max_power } => {
				let energy = lerp(energy_at_min_power, energy_at_max_power, t);
				debug_assert!(energy >= 0.0);
				sample_normal_value(energy, cv)
			}
			Quadratic { t_optimum, energy_at_optimum } => {
				let energy = quadratic_value(t_optimum, energy_at_optimum, t);
                debug_assert!(energy >= 0.0);
				sample_normal_value(energy, cv)
			}
			Sigmoid { min_energy, max_energy, t_middle, steepness } => {
				let energy = sigmoid_value(min_energy, max_energy, t_middle, steepness, t);
				debug_assert!(energy >= 0.0);
				sample_normal_value(energy, cv)
			}
		}
	}
}

impl RuntimeCurve {
	fn eval(self, t: f32, cv: f32) -> f32 {
        debug_assert!(t >= 0.0 && t <= 1.0);
        use RuntimeCurve::*;
		match self {
			Linear { runtime_at_min_power, runtime_at_max_power } => {
				let runtime = lerp(runtime_at_min_power, runtime_at_max_power, t);
				debug_assert!(runtime >= 0.0);
				sample_normal_value(runtime, cv)
			}
			Quadratic { t_optimum, runtime_at_optimum } => {
				let runtime = quadratic_value(t_optimum, runtime_at_optimum, t);
                debug_assert!(runtime >= 0.0);
				sample_normal_value(runtime, cv)
			}
			Sigmoid { min_runtime, max_runtime, t_middle, steepness } => {
				let runtime = sigmoid_value(min_runtime, max_runtime, t_middle, steepness, t);
				debug_assert!(runtime >= 0.0);
				sample_normal_value(runtime, cv)
			}
		}
	}
}

fn run(
	best_score: f32,
	config: &GeneticControllerConfig,
	energy_curve: EnergyCurve,
	runtime_curve: RuntimeCurve,
	energy_cv: f32,
	runtime_cv: f32,
	convergence_score_threshold: f32,
) -> Option<usize> {
	let mut controller = GeneticController::new(config.clone(), &Capabilities::default());
	let mut recent_score_error_ratios = vec![f32::INFINITY; CONVERGENCE_WINDOW];
	let mut recent_score_error_index = 0;

	for iteration in 1..=MAX_ITERATIONS {
		let demand = controller.get_demand();
		let t = demand.powercap_pct;

		let energy = energy_curve.eval(t, energy_cv);
		let runtime = runtime_curve.eval(t, runtime_cv);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = score(&sample, config.energy_preference);
		let score_error_ratio = (score - best_score).abs() / best_score.abs().max(f32::EPSILON);
		recent_score_error_ratios[recent_score_error_index] = score_error_ratio;
		recent_score_error_index = (recent_score_error_index + 1) % CONVERGENCE_WINDOW;

		controller.push_sample(sample);

		if iteration >= CONVERGENCE_WINDOW
			&& has_converged(&recent_score_error_ratios, convergence_score_threshold)
		{
			return Some(iteration)
		}
	}

	// Did not converge
	None
}

fn main() {
    let Args {
		energy_cv,
		runtime_cv,
		energy_curve,
		runtime_curve,
        config,
    } = Args::parse();

	let convergence_score_threshold = derive_score_error_threshold(
		config.energy_preference,
		energy_cv,
		runtime_cv,
		CONVERGENCE_THRESHOLD_MULTIPLIER,
	);

    // Precompute the optimal powercap for the given curves and score definition,
    // to have a reference for the controller's performance.
	let (best_score, _best_powercap, _best_energy, _best_runtime) = find_optimal_powercap(
		runtime_curve,
		energy_curve,
		config.energy_preference,
		config.power_min,
		config.power_max,
	);

	const RUNS: usize = 200;
	let mut runs = vec![usize::MAX; RUNS];
	for i in 0..RUNS {
		let converged = run(
			best_score,
			&config,
			energy_curve,
			runtime_curve,
			energy_cv,
			runtime_cv,
			convergence_score_threshold,
		);

		if let Some(iterations) = converged {
			runs[i] = iterations;
		}
	}

	let (median, q1, q3) = quartiles(runs);
	println!("Iterations until convergence over {} runs: median={}, Q1={}, Q3={}",
		RUNS, median, q1, q3);
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
///
/// Returns (best_powercap, best_score).
fn find_optimal_powercap(
	runtime_curve: RuntimeCurve,
	energy_curve: EnergyCurve,
	alpha: f32,
	power_min: f32,
	power_max: f32,
) -> (f32, f32, f32, f32) {
    let mut best_score = f32::INFINITY;
    let mut best_powercap = power_min;
    let mut best_energy = f32::INFINITY;
    let mut best_runtime = f32::INFINITY;

	for i in 0..=5000 {
		let t = i as f32 / 5000.0;
		let powercap = lerp(power_min, power_max, t);
        let energy=  energy_curve.eval(powercap, 0.0);
        let runtime = runtime_curve.eval(powercap, 0.0);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = score(&sample, alpha);
		if score < best_score {
			best_score = score;
			best_powercap = powercap;
            best_energy = energy;
            best_runtime = runtime;
		}
	}

	(best_score, best_powercap, best_energy, best_runtime)
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}

/// Build a smooth quadratic curve with one optimum.
///
/// The curve bottoms out at `t_optimum`, where it returns the value at the
/// optimum. Away from that point, it follows a normalized parabola of the form
/// `1 + x^2`, where `x` is the distance from the optimum scaled so that the
/// farthest end of the interval maps to `x = 1`.
fn quadratic_value(t_optimum: f32, t_optimum_value: f32, t: f32) -> f32 {
	let max_distance = t_optimum.max(1.0 - t_optimum).max(f32::EPSILON);
	let distance = (t - t_optimum).abs();
	let normalized_distance = (distance / max_distance).min(1.0);
	let shape = 1.0 + normalized_distance.powi(2);
	t_optimum_value * shape
}

/// Build a smooth sigmoid-like curve with plateaus at both ends.
///
/// `t_middle` marks the center of the S-curve. `min_value` and `max_value`
/// define the plateau levels at the edges of the interval, and `steepness`
/// controls how abruptly the curve transitions between them.
///
/// Steepness is interpreted on a `0..=1` scale:
/// - `0` is almost linear
/// - `1` is very steep, approaching a sign-like transition
///
/// The curve uses a shifted hyperbolic tangent, which keeps the code simple
/// while still producing a smooth S-shaped transition between the two levels.
fn sigmoid_value(min_value: f32, max_value: f32, t_middle: f32, steepness: f32, t: f32) -> f32 {
	debug_assert!(t >= 0.0 && t <= 1.0);
	debug_assert!(t_middle >= 0.0 && t_middle <= 1.0);
	debug_assert!(steepness >= 0.0 && steepness <= 1.0);

	let curve_strength = 2.0 + steepness * 10.0;
	let normalized = 0.5 * (1.0 + ((t - t_middle) * curve_strength).tanh());
	lerp(min_value, max_value, normalized)
}

fn sample_normal_value(mean: f32, cv: f32) -> f32 {
	if cv <= 0.0 {
		return mean;
	}
	let std = mean * cv;
	let mut rng = rand::rng();
	let normal = Normal::new(mean, std).unwrap();
	let v = normal.sample(&mut rng);
    debug_assert!(v >= 0.0);
    v
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
fn derive_score_error_threshold(
	energy_preference: f32,
	energy_cv: f32,
	runtime_cv: f32,
	threshold_multiplier: f32,
) -> f32 {
	let runtime_preference = 1.0 - energy_preference;
	let derived_score_cv = f32::sqrt(
        (energy_preference * energy_cv).powi(2)
        + (runtime_preference * runtime_cv).powi(2)
    );
	derived_score_cv * threshold_multiplier
}

fn has_converged(recent_score_error_ratios: &[f32], convergence_score_threshold: f32) -> bool {
	debug_assert_eq!(recent_score_error_ratios.len(), CONVERGENCE_WINDOW);
	let num_converged = recent_score_error_ratios
		.iter()
		.filter(|&&score_error_ratio| score_error_ratio <= convergence_score_threshold)
		.count();
	num_converged >= CONVERGENCE_REQUIRED
}

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
