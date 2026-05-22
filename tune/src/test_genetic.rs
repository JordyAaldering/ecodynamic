use clap::Parser;
use controller::*;
use prelude::*;

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
    #[arg(long, default_value_t = 0.005)]
    runtime_cv: f32,

	#[arg(long, default_value = "Quadratic:0.1,0.2,3.0")]
    energy_curve: Curve,
	#[arg(long, default_value = "Sigmoid:0.8,0.9,0.3,-100.0")]
    runtime_curve: Curve,

    #[command(flatten)]
    config: GeneticControllerConfig,
}

fn main() {
    env_logger::init();

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
	let (best_score, best_energy, best_runtime, best_powercap) = find_optimal_powercap(
		config.energy_preference,
		energy_curve,
		runtime_curve,
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
		log::trace!("iter={:03} powercap={t:.4} score={:.4} error_pct={:.2}% dist_to_optimal={:.4}",
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

fn has_converged(recent_score_error_ratios: &[f32], convergence_score_threshold: f32) -> bool {
	debug_assert_eq!(recent_score_error_ratios.len(), CONVERGENCE_WINDOW);
	let num_converged = recent_score_error_ratios
		.iter()
		.filter(|&&score_error_ratio| score_error_ratio <= convergence_score_threshold)
		.count();
	num_converged >= CONVERGENCE_REQUIRED
}
