use clap::Parser;
use controller::*;
use prelude::*;

const INITIALIZATION_ITERATIONS: usize = 200;
const MAX_ADAPTATION_ITERATIONS: usize = 400;
/// We consider the controller converged once enough recent iterations stay close to the
/// predicted best score: at least CONVERGENCE_REQUIRED of the last CONVERGENCE_WINDOW
/// iterations must be within a derived score-error threshold of that best score.
const CONVERGENCE_WINDOW: usize = 20;
const CONVERGENCE_REQUIRED: usize = 15;
const CONVERGENCE_THRESHOLD_MULTIPLIER: f32 = 1.5;

// Warm-up curves used to place the controller near a known initial optimum.
const INIT_ENERGY_CURVE: Curve = Curve::Quadratic {
    lb: 0.1,
    t_middle: 0.25,
    steepness: 3.0,
};

const INIT_RUNTIME_CURVE: Curve = Curve::Quadratic {
    lb: 0.1,
    t_middle: 0.75,
    steepness: 3.0,
};

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

fn run(
    target_best_score: f32,
    config: &GeneticControllerConfig,
    energy_curve: Curve,
    runtime_curve: Curve,
    energy_cv: f32,
    runtime_cv: f32,
    convergence_score_threshold: f32,
) -> Option<usize> {
    let mut controller = GeneticController::new(config.clone(), &Capabilities::default());

    for _ in 1..=INITIALIZATION_ITERATIONS {
        let demand = controller.get_demand();
        let t = demand.powercap_pct;
        let energy = INIT_ENERGY_CURVE.eval(t, energy_cv);
        let runtime = INIT_RUNTIME_CURVE.eval(t, runtime_cv);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };
        controller.push_sample(sample);
    }

    let mut recent_score_error_ratios = vec![f32::INFINITY; CONVERGENCE_WINDOW];
    let mut recent_score_error_index = 0;

    for iteration in 1..=MAX_ADAPTATION_ITERATIONS {
        let demand = controller.get_demand();
        let t = demand.powercap_pct;

        let energy = energy_curve.eval(t, energy_cv);
        let runtime = runtime_curve.eval(t, runtime_cv);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

        let score = score(&sample, config.energy_preference);
        let score_error_ratio = (score - target_best_score).abs() / target_best_score.abs().max(f32::EPSILON);
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

    // Compute the optimum for the post-shift workload.
    let (target_best_score, _, _, _) = find_optimal_powercap(
        config.energy_preference,
        energy_curve,
        runtime_curve,
        config.power_min,
        config.power_max,
    );

    const RUNS: usize = 200;
	// usize::MAX / 2, to avoid an overflow in the median calculation
    let mut runs = vec![usize::MAX / 2; RUNS];
    for i in 0..RUNS {
        let converged = run(
            target_best_score,
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
    println!("f(\\x) = {};", energy_curve.to_tikz());
    println!("g(\\x) = {};", runtime_curve.to_tikz());
    if median == usize::MAX / 2 {
        println!("Did not converge in (most) runs");
    } else {
        println!("Iterations until re-convergence over {} runs: median={}, Q1={}, Q3={}", RUNS, median, q1, q3);
    }
}

fn has_converged(recent_score_error_ratios: &[f32], convergence_score_threshold: f32) -> bool {
    debug_assert_eq!(recent_score_error_ratios.len(), CONVERGENCE_WINDOW);
    let num_converged = recent_score_error_ratios
        .iter()
        .filter(|&&score_error_ratio| score_error_ratio <= convergence_score_threshold)
        .count();
    num_converged >= CONVERGENCE_REQUIRED
}
