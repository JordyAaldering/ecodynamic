use clap::Parser;
use controller::*;

const NUM_ITERATIONS: usize = 100;

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

	let runtime_curve = RuntimeCurve::Linear { runtime_at_min_power: 100.0, runtime_at_max_power: 25.0, measurement_delta: 0.5 };
	let energy_curve = EnergyCurve::Linear { energy_at_min_power: 20.0, energy_at_max_power: 90.0, measurement_delta: 2.0 };

    // Precompute the optimal powercap for the given curves and score definition,
    // to have a reference for the controller's performance.
	let (best_powercap, best_score) = find_optimal_powercap(
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

	for iteration in 1..=NUM_ITERATIONS {
		let demand = controller.get_demand();
		let t = demand.powercap_pct;

        let sample = Sample {
			region_uid: 0,
			energy: energy_curve.eval(t),
			runtime: runtime_curve.eval(t),
			usertime: None,
		};

		let score = score(&sample, e_pref);
		if score < best_observed_score {
			best_observed_powercap = t;
			best_observed_score = score;
		}

		controller.push_sample(sample);

		let distance_to_optimum = (t - best_powercap).abs();
		println!(
			"iter={iteration:03} powercap={t:.4} score={score:.4} dist_to_optimal={distance_to_optimum:.4}"
		);
	}

	println!();
	println!("Expected optimum: powercap={:.4}, score={:.4}", best_powercap, best_score);
	println!("Best observed:    powercap={:.4}, score={:.4}", best_observed_powercap, best_observed_score);
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
) -> (f32, f32) {
    let mut best_powercap = power_min;
    let mut best_score = f32::INFINITY;

	for i in 0..=5000 {
		let t = i as f32 / 5000.0;
		let powercap = lerp(power_min, power_max, t);
        let sample = Sample {
            region_uid: 0,
            energy: energy_curve.eval(powercap),
            runtime: runtime_curve.eval(powercap),
            usertime: None,
        };

		let score = score(&sample, alpha);
		if score < best_score {
			best_powercap = powercap;
			best_score = score;
		}
	}

	(best_powercap, best_score)
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
