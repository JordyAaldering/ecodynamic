use clap::Parser;
use controller::*;

const MAX_ITERATIONS: usize = 200;
/// We consider the controller converged once enough recent iterations stay close to the
/// predicted best score: at least CONVERGENCE_REQUIRED_N of the last CONVERGENCE_WINDOW_M
/// iterations must be within CONVERGENCE_SCORE_PERCENT of that best score.
const CONVERGENCE_WINDOW_M: usize = 10;
const CONVERGENCE_REQUIRED_N: usize = 7;
const CONVERGENCE_SCORE_PERCENT: f32 = 0.10;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[command(flatten)]
    pub config: GeneticControllerConfig,
}

#[derive(Clone, Copy, Debug)]
enum EnergyCurve {
	Linear {
        energy_at_min_power: f32,
        energy_at_max_power: f32,
        measurement_delta: f32,
    },
}

#[derive(Clone, Copy, Debug)]
enum RuntimeCurve {
	Linear {
        runtime_at_min_power: f32,
        runtime_at_max_power: f32,
        measurement_delta: f32,
    },
}

impl EnergyCurve {
	fn eval(self, t: f32) -> f32 {
        use EnergyCurve::*;
		match self {
			Linear { energy_at_min_power: min_powercap_energy, energy_at_max_power: max_powercap_energy, measurement_delta } => {
				let energy = lerp(min_powercap_energy, max_powercap_energy, t);
                energy + rand::random_range(-measurement_delta..=measurement_delta)
			}
		}
	}
}

impl RuntimeCurve {
	fn eval(self, t: f32) -> f32 {
        use RuntimeCurve::*;
		match self {
			Linear { runtime_at_min_power: min_powercap_runtime, runtime_at_max_power: max_powercap_runtime, measurement_delta } => {
				let runtime = lerp(min_powercap_runtime, max_powercap_runtime, t);
                runtime + rand::random_range(-measurement_delta..=measurement_delta)
			}
		}
	}
}

fn main() {
    let Args { config } = Args::parse();

	let runtime_curve = RuntimeCurve::Linear { runtime_at_min_power: 50.0, runtime_at_max_power: 20.0, measurement_delta: 0.1 };
	let energy_curve = EnergyCurve::Linear { energy_at_min_power: 60.0, energy_at_max_power: 90.0, measurement_delta: 2.0 };

    // Precompute the optimal powercap for the given curves and score definition,
    // to have a reference for the controller's performance.
	let (best_powercap, best_energy, best_runtime, best_score) = find_optimal_powercap(
		runtime_curve,
		energy_curve,
		config.energy_preference,
		config.power_min,
		config.power_max,
	);

    let e_pref = config.energy_preference;
	let mut controller = GeneticController::new(config, &Capabilities::default());
    let mut best_observed_powercap = 0.0;
    let mut best_observed_score = f32::INFINITY;

	let mut total_energy_overhead = 0.0;
	let mut total_runtime_overhead = 0.0;
	let mut recent_score_error_ratios = vec![f32::INFINITY; CONVERGENCE_WINDOW_M];
	let mut recent_score_error_index = 0;
	let mut iterations_done = 0;
    let mut converged = false;

	for iteration in 1..=MAX_ITERATIONS {
		iterations_done = iteration;
		let demand = controller.get_demand();
		let t = demand.powercap_pct;

        let energy = energy_curve.eval(t);
        let runtime = runtime_curve.eval(t);
        total_energy_overhead += energy - best_energy;
        total_runtime_overhead += runtime - best_runtime;
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = score(&sample, e_pref);
		let score_error_ratio = (score - best_score).abs() / best_score.abs().max(f32::EPSILON);
		recent_score_error_ratios[recent_score_error_index] = score_error_ratio;
		recent_score_error_index = (recent_score_error_index + 1) % CONVERGENCE_WINDOW_M;
		if score < best_observed_score {
			best_observed_powercap = t;
			best_observed_score = score;
		}

		controller.push_sample(sample);

		let distance_to_optimum = (t - best_powercap).abs();
		println!("iter={iteration:03} powercap={t:.4} score={score:.4} error_pct={:.2}% dist_to_optimal={distance_to_optimum:.4}",
			score_error_ratio * 100.0);

		if iteration >= CONVERGENCE_WINDOW_M && has_converged(&recent_score_error_ratios) {
			converged = true;
			break;
		}
	}

    if converged {
		println!("Converged after {iterations_done} iterations: {CONVERGENCE_REQUIRED_N} of the last {CONVERGENCE_WINDOW_M} score errors were within {CONVERGENCE_SCORE_PERCENT:.2}% of the predicted best score");
    } else {
        println!("Did not converge");
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
/// Returns (best_powercap, best_score).
fn find_optimal_powercap(
	runtime_curve: RuntimeCurve,
	energy_curve: EnergyCurve,
	alpha: f32,
	power_min: f32,
	power_max: f32,
) -> (f32, f32, f32, f32) {
    let mut best_powercap = power_min;
    let mut best_energy = f32::INFINITY;
    let mut best_runtime = f32::INFINITY;
    let mut best_score = f32::INFINITY;

	for i in 0..=5000 {
		let t = i as f32 / 5000.0;
		let powercap = lerp(power_min, power_max, t);
        let energy=  energy_curve.eval(powercap);
        let runtime = runtime_curve.eval(powercap);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

		let score = score(&sample, alpha);
		if score < best_score {
			best_powercap = powercap;
            best_energy = energy;
            best_runtime = runtime;
			best_score = score;
		}
	}

	(best_powercap, best_energy, best_runtime, best_score)
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}

fn has_converged(recent_score_error_ratios: &[f32]) -> bool {
	if recent_score_error_ratios.len() < CONVERGENCE_WINDOW_M {
		return false;
	}

	recent_score_error_ratios
		.iter()
		.filter(|&&score_error_ratio| score_error_ratio <= CONVERGENCE_SCORE_PERCENT)
		.count()
		>= CONVERGENCE_REQUIRED_N
}
