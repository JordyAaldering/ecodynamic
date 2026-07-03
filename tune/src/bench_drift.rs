use clap::Parser;
use controller::*;
use prelude::*;

/// Benchmark to verify that the genetic algorithm can track a monotonically
/// shifting workload via mutations (without triggering immigration).
///
/// The workload gradually shifts over time: the energy curve's optimum slowly
/// moves, requiring the controller to continuously adjust its recommended powercap.
/// This tests that:
/// 1. The controller tracks the moving optimum (stays near it)
/// 2. Immigration does NOT trigger (the shift is gradual enough)

const NUM_ITERATIONS: usize = 1000;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(short('i'), long, default_value_t = 100)]
    runs: usize,

    /// Coefficient of variation for energy measurements.
    #[arg(long, default_value_t = 0.025)]
    energy_cv: f32,
    #[arg(long, default_value_t = 0.005)]
    runtime_cv: f32,

    #[command(flatten)]
    config: GeneticControllerConfig,
}

struct TestCase {
    name: &'static str,
    /// Total shift in the curve's center parameter over TOTAL_ITERATIONS
    drift_amount: f32,
    /// Base energy curve (will be shifted over time)
    energy_base: DriftingCurve,
    /// Base runtime curve (will be shifted over time)
    runtime_base: DriftingCurve,
}

/// A curve that drifts linearly over time.
#[derive(Clone, Copy, Debug)]
enum DriftingCurve {
    /// Quadratic with drifting t_middle
    Quadratic { lb: f32, t_middle_start: f32, steepness: f32 },
    /// Sigmoid with drifting t_middle
    Sigmoid { lb: f32, ub: f32, t_middle_start: f32, steepness: f32 },
}

impl DriftingCurve {
    fn at_time(&self, progress: f32, drift_amount: f32) -> Curve {
        let offset = progress * drift_amount;
        match *self {
            DriftingCurve::Quadratic { lb, t_middle_start, steepness } => {
                let t_middle = (t_middle_start + offset).clamp(0.05, 0.95);
                let spec = format!("Quadratic:{},{},{}", lb, t_middle, steepness);
                spec.parse().unwrap()
            }
            DriftingCurve::Sigmoid { lb, ub, t_middle_start, steepness } => {
                let t_middle = (t_middle_start + offset).clamp(0.05, 0.95);
                let spec = format!("Sigmoid:{},{},{},{}", lb, ub, t_middle, steepness);
                spec.parse().unwrap()
            }
        }
    }
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "Quad slow drift (+0.1)",
            drift_amount: 0.1,
            energy_base: DriftingCurve::Quadratic { lb: 0.1, t_middle_start: 0.2, steepness: 3.0 },
            runtime_base: DriftingCurve::Quadratic { lb: 0.1, t_middle_start: 0.75, steepness: 3.0 },
        },
        TestCase {
            name: "Quad moderate drift (+0.2)",
            drift_amount: 0.2,
            energy_base: DriftingCurve::Quadratic { lb: 0.1, t_middle_start: 0.2, steepness: 3.0 },
            runtime_base: DriftingCurve::Quadratic { lb: 0.1, t_middle_start: 0.75, steepness: 3.0 },
        },
        TestCase {
            name: "Quad fast drift (+0.4)",
            drift_amount: 0.4,
            energy_base: DriftingCurve::Quadratic { lb: 0.1, t_middle_start: 0.2, steepness: 3.0 },
            runtime_base: DriftingCurve::Quadratic { lb: 0.1, t_middle_start: 0.6, steepness: 3.0 },
        },
        TestCase {
            name: "Sigmoid slow drift (+0.1)",
            drift_amount: 0.1,
            energy_base: DriftingCurve::Sigmoid { lb: 0.1, ub: 0.8, t_middle_start: 0.3, steepness: 15.0 },
            runtime_base: DriftingCurve::Sigmoid { lb: 0.8, ub: 0.1, t_middle_start: 0.7, steepness: -15.0 },
        },
        TestCase {
            name: "Sigmoid moderate drift (+0.2)",
            drift_amount: 0.2,
            energy_base: DriftingCurve::Sigmoid { lb: 0.1, ub: 0.8, t_middle_start: 0.3, steepness: 15.0 },
            runtime_base: DriftingCurve::Sigmoid { lb: 0.8, ub: 0.1, t_middle_start: 0.7, steepness: -15.0 },
        },
        TestCase {
            name: "Sigmoid fast drift (+0.4)",
            drift_amount: 0.4,
            energy_base: DriftingCurve::Sigmoid { lb: 0.1, ub: 0.8, t_middle_start: 0.3, steepness: 15.0 },
            runtime_base: DriftingCurve::Sigmoid { lb: 0.8, ub: 0.1, t_middle_start: 0.6, steepness: -15.0 },
        },
    ]
}

struct RunResult {
    immigration_count: usize,
    /// Average score error relative to the current optimum across the last half of the run
    avg_tracking_error: f32,
}

fn run(
    config: &GeneticControllerConfig,
    case: &TestCase,
    energy_cv: f32,
    runtime_cv: f32,
) -> RunResult {
    let mut controller = GeneticController::new(config.clone(), &Capabilities::default());
    let mut immigration_count = 0;
    let mut tracking_errors = Vec::new();

    for iteration in 0..NUM_ITERATIONS {
        let progress = iteration as f32 / NUM_ITERATIONS as f32;

        let energy_curve = case.energy_base.at_time(progress, case.drift_amount);
        let runtime_curve = case.runtime_base.at_time(progress, case.drift_amount);

        let demand = controller.get_demand();
        let t = demand.powercap_pct;

        let energy = energy_curve.eval(t, energy_cv);
        let runtime = runtime_curve.eval(t, runtime_cv);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

        // Compute optimal score for the current (drifted) curves
        let (best_score, _, _, _) = find_optimal_powercap(
            config.energy_preference,
            energy_curve,
            runtime_curve,
            config.power_min,
            config.power_max,
        );

        let score = score(&sample, config.energy_preference);
        let score_error = (score - best_score).abs() / best_score.abs().max(f32::EPSILON);

        // Track errors in the second half of the run (after initial convergence)
        if iteration >= NUM_ITERATIONS / 2 {
            tracking_errors.push(score_error);
        }

        controller.push_sample(sample);

        if controller.immigration_triggered() {
            immigration_count += 1;
        }
    }

    let avg_tracking_error = if tracking_errors.is_empty() {
        f32::INFINITY
    } else {
        tracking_errors.iter().sum::<f32>() / tracking_errors.len() as f32
    };

    RunResult {
        immigration_count,
        avg_tracking_error,
    }
}

fn main() {
    env_logger::init();

    let Args {
        runs,
        energy_cv,
        runtime_cv,
        mut config,
    } = Args::parse();

    config.threads_min = 1.0;
    config.threads_max = 1.0;

    let cases = get_test_cases();

    println!("Monotonic Drift Benchmark: Gradual Workload Shift Tracking");
    println!("===========================================================");
    println!("Configuration: pop={}, sr={}, mr={}, ms={}, decay={}, e_pref={}",
        config.population_size, config.survival_rate, config.mutation_rate,
        config.mutation_strength, config.mutation_rate_decay, config.energy_preference);
    println!("Iterations per run: {}, Runs per case: {}", NUM_ITERATIONS, runs);
    println!();
    println!("┌─────────────────────────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│ Test Case                       │ Avg Track %  │ Immigr. Rate │ Avg Triggers │");
    println!("├─────────────────────────────────┼──────────────┼──────────────┼──────────────┤");

    for case in &cases {
        let mut total_tracking_error = 0.0;
        let mut total_immigrations = 0;
        let mut runs_with_immigration = 0;

        for _ in 0..runs {
            let result = run(&config, case, energy_cv, runtime_cv);
            total_tracking_error += result.avg_tracking_error;
            total_immigrations += result.immigration_count;
            if result.immigration_count > 0 {
                runs_with_immigration += 1;
            }
        }

        let avg_tracking_error = total_tracking_error / runs as f32 * 100.0;
        let immigr_rate = runs_with_immigration as f32 / runs as f32 * 100.0;
        let avg_triggers = total_immigrations as f32 / runs as f32;

        println!(
            "│ {:<31} │ {:>8.2}%    │ {:>8.1}%    │ {:>10.2}  │",
            case.name, avg_tracking_error, immigr_rate, avg_triggers
        );
    }

    println!("└─────────────────────────────────┴──────────────┴──────────────┴──────────────┘");
    println!();
    println!("Avg Track % = average score error relative to the shifting optimum (second half of run).");
    println!("Immigration Rate = percentage of runs that had at least one immigration event.");
    println!("Target: Low tracking error (<10%) with minimal immigration for slow/moderate drifts.");
}
