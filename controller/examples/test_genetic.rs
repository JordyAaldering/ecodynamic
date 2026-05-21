use clap::Parser;
use controller::*;
use rand::distr::Distribution;
use rand_distr::Normal;

const MAX_ITERATIONS: usize = 200;
/// We consider the controller converged once enough recent iterations stay close to the
/// predicted best score: at least CONVERGENCE_REQUIRED of the last CONVERGENCE_WINDOW
/// iterations must be within a derived score-error threshold of that best score.
const CONVERGENCE_WINDOW: usize = 20;
const CONVERGENCE_REQUIRED: usize = 15;
const CONVERGENCE_THRESHOLD_MULTIPLIER: f32 = 2.0;

#[derive(Clone, Debug, Parser)]
pub struct Args {
	/// Coefficient of variation for energy measurements.
    ///
    /// Example, if the CV is 0.05 (5%) and the expected energy is 100J, then
    /// for a normal distribution about 68% of measurements will be in the
    /// range [95J, 105J], and about 95% will be in the range [90J, 110J].
    #[arg(long, default_value_t = 0.025)]
    pub energy_cv: f32,
	/// Coefficient of variation for runtime measurements.
    #[arg(long, default_value_t = 0.005)]
    pub runtime_cv: f32,

    #[command(flatten)]
    pub config: GeneticControllerConfig,
}

#[derive(Clone, Copy, Debug)]
enum EnergyCurve {
	Linear {
		energy_at_min_power: f32,
		energy_at_max_power: f32,
	},
	Quadratic {
		t_optimum: f32,
		energy_at_optimum: f32,
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
		}
	}
}

fn main() {
    let Args {
		energy_cv,
		runtime_cv,
        config,
    } = Args::parse();

	let runtime_curve = RuntimeCurve::Quadratic { runtime_at_optimum: 20.0, t_optimum: 0.25 };
	let energy_curve = EnergyCurve::Quadratic { energy_at_optimum: 60.0, t_optimum: 0.75 };
	let convergence_score_threshold = derive_score_error_threshold(
		config.energy_preference,
		energy_cv,
		runtime_cv,
		CONVERGENCE_THRESHOLD_MULTIPLIER,
	);

    // Precompute the optimal powercap for the given curves and score definition,
    // to have a reference for the controller's performance.
	let (best_score, best_powercap, best_energy, best_runtime) = find_optimal_powercap(
		runtime_curve,
		energy_curve,
		config.energy_preference,
		config.power_min,
		config.power_max,
	);

    let e_pref = config.energy_preference;
	let mut controller = GeneticController::new(config, &Capabilities::default());
    let mut best_observed_score = f32::INFINITY;
    let mut best_observed_powercap = 0.0;

	let mut total_energy_overhead = 0.0;
	let mut total_runtime_overhead = 0.0;
	let mut recent_score_error_ratios = vec![f32::INFINITY; CONVERGENCE_WINDOW];
	let mut recent_score_error_index = 0;
	let mut iterations_done = 0;
    let mut converged = false;

	for iteration in 1..=MAX_ITERATIONS {
		iterations_done = iteration;
		let demand = controller.get_demand();
		let t = demand.powercap_pct;

		let energy = energy_curve.eval(t, energy_cv);
		let runtime = runtime_curve.eval(t, runtime_cv);
        total_energy_overhead += energy - best_energy;
        total_runtime_overhead += runtime - best_runtime;
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = score(&sample, e_pref);
		let score_error_ratio = (score - best_score).abs() / best_score.abs().max(f32::EPSILON);
		recent_score_error_ratios[recent_score_error_index] = score_error_ratio;
		recent_score_error_index = (recent_score_error_index + 1) % CONVERGENCE_WINDOW;
		if score < best_observed_score {
			best_observed_powercap = t;
			best_observed_score = score;
		}

		controller.push_sample(sample);

		let distance_to_optimum = (t - best_powercap).abs();
		println!("iter={:03} powercap={t:.4} score={:.4} error_pct={:.2}% dist_to_optimal={:.4}",
			iteration, score, score_error_ratio * 100.0, distance_to_optimum);

		if iteration >= CONVERGENCE_WINDOW
			&& has_converged(&recent_score_error_ratios, convergence_score_threshold) {
			converged = true;
			break;
		}
	}

    if converged {
		println!("Converged after {} iterations ({} of the last {} iterations within score threshold {:.2}%)",
			iterations_done, CONVERGENCE_REQUIRED, CONVERGENCE_WINDOW, convergence_score_threshold * 100.0);
    } else {
        println!("Did not converge (score threshold {:.2}%)",
            convergence_score_threshold * 100.0);
    }

	println!("Expected optimum: powercap={:.4}W, score={:.4}", best_powercap, best_score);
	println!("Best observed:    powercap={:.4}W, score={:.4}", best_observed_powercap, best_observed_score);
	let total_expected_energy = iterations_done as f32 * best_energy;
	let total_expected_runtime = iterations_done as f32 * best_runtime;
    let energy_overhead_pct = total_energy_overhead / total_expected_energy * 100.0;
    let runtime_overhead_pct = total_runtime_overhead / total_expected_runtime * 100.0;
    println!("Total overhead:   energy={:.2}%, runtime={:.2}%", energy_overhead_pct, runtime_overhead_pct);
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
