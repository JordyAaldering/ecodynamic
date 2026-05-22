use clap::Parser;
use controller::*;
use prelude::*;

/// Benchmark to verify that immigration does NOT trigger under stable workloads
/// with normal measurement noise. This tests for false positives in the shift
/// detection mechanism.
///
/// For each test case, the controller runs for many generations on a fixed workload
/// (with measurement noise). We count how often immigration is falsely triggered.
/// A well-tuned system should have a false positive rate near 0%.

const TOTAL_ITERATIONS: usize = 500;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(short('i'), long, default_value_t = 200)]
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
    energy_curve: Curve,
    runtime_curve: Curve,
    energy_cv: f32,
    runtime_cv: f32,
}

fn get_test_cases(default_energy_cv: f32, default_runtime_cv: f32) -> Vec<TestCase> {
    vec![
        // Standard noise levels
        TestCase {
            name: "Quadratic (low noise)",
            energy_curve: "Quadratic:0.1,0.2,3.0".parse().unwrap(),
            runtime_curve: "Quadratic:0.1,0.8,3.0".parse().unwrap(),
            energy_cv: default_energy_cv,
            runtime_cv: default_runtime_cv,
        },
        TestCase {
            name: "Sigmoid (low noise)",
            energy_curve: "Sigmoid:0.1,0.8,0.3,20.0".parse().unwrap(),
            runtime_curve: "Sigmoid:0.8,0.1,0.7,-20.0".parse().unwrap(),
            energy_cv: default_energy_cv,
            runtime_cv: default_runtime_cv,
        },
        TestCase {
            name: "Linear (low noise)",
            energy_curve: "Linear:0.2,0.8".parse().unwrap(),
            runtime_curve: "Linear:0.8,0.2".parse().unwrap(),
            energy_cv: default_energy_cv,
            runtime_cv: default_runtime_cv,
        },
        // Higher noise — this is the harder case for false positives
        TestCase {
            name: "Quadratic (high noise)",
            energy_curve: "Quadratic:0.1,0.2,3.0".parse().unwrap(),
            runtime_curve: "Quadratic:0.1,0.8,3.0".parse().unwrap(),
            energy_cv: 0.08,
            runtime_cv: 0.03,
        },
        TestCase {
            name: "Sigmoid (high noise)",
            energy_curve: "Sigmoid:0.1,0.8,0.3,20.0".parse().unwrap(),
            runtime_curve: "Sigmoid:0.8,0.1,0.7,-20.0".parse().unwrap(),
            energy_cv: 0.08,
            runtime_cv: 0.03,
        },
        TestCase {
            name: "Linear (high noise)",
            energy_curve: "Linear:0.2,0.8".parse().unwrap(),
            runtime_curve: "Linear:0.8,0.2".parse().unwrap(),
            energy_cv: 0.08,
            runtime_cv: 0.03,
        },
        // Very high noise — stress test
        TestCase {
            name: "Quadratic (extreme noise)",
            energy_curve: "Quadratic:0.1,0.4,2.0".parse().unwrap(),
            runtime_curve: "Quadratic:0.1,0.7,2.0".parse().unwrap(),
            energy_cv: 0.15,
            runtime_cv: 0.06,
        },
        TestCase {
            name: "Linear (extreme noise)",
            energy_curve: "Linear:0.2,0.5".parse().unwrap(),
            runtime_curve: "Linear:0.5,0.2".parse().unwrap(),
            energy_cv: 0.15,
            runtime_cv: 0.06,
        },
    ]
}

/// Run a single trial and return the number of immigration events observed.
fn run(
    config: &GeneticControllerConfig,
    energy_curve: Curve,
    runtime_curve: Curve,
    energy_cv: f32,
    runtime_cv: f32,
) -> usize {
    let mut controller = GeneticController::new(config.clone(), &Capabilities::default());
    let mut immigration_count = 0;
    let mut prev_generation = 0;

    for _ in 0..TOTAL_ITERATIONS {
        let demand = controller.get_demand();
        let t = demand.powercap_pct;

        let energy = energy_curve.eval(t, energy_cv);
        let runtime = runtime_curve.eval(t, runtime_cv);
        let sample = Sample { region_uid: 0, energy, runtime, usertime: None };

        controller.push_sample(sample);

        // Detect immigration by checking if generation advanced and population was replaced
        // We use a simple proxy: track generation changes via the public interface
        let current_generation = controller.generation();
        if current_generation > prev_generation {
            if controller.immigration_triggered() {
                immigration_count += 1;
            }
            prev_generation = current_generation;
        }
    }

    immigration_count
}

fn main() {
    env_logger::init();

    let Args {
        runs,
        energy_cv,
        runtime_cv,
        config,
    } = Args::parse();

    let cases = get_test_cases(energy_cv, runtime_cv);

    println!("Stability Benchmark: False Positive Immigration Detection");
    println!("=========================================================");
    println!("Configuration: pop={}, sr={}, mr={}, ms={}, e_pref={}",
        config.population_size, config.survival_rate, config.mutation_rate,
        config.mutation_strength, config.energy_preference);
    println!("Iterations per run: {}, Runs per case: {}", TOTAL_ITERATIONS, runs);
    println!();
    println!("┌─────────────────────────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│ Test Case                       │ FP Rate      │ Avg Triggers │ Max Triggers │");
    println!("├─────────────────────────────────┼──────────────┼──────────────┼──────────────┤");

    for case in &cases {
        let mut total_triggers = 0;
        let mut runs_with_triggers = 0;
        let mut max_triggers = 0;

        for _ in 0..runs {
            let triggers = run(&config, case.energy_curve, case.runtime_curve, case.energy_cv, case.runtime_cv);
            total_triggers += triggers;
            if triggers > 0 {
                runs_with_triggers += 1;
            }
            max_triggers = max_triggers.max(triggers);
        }

        let fp_rate = runs_with_triggers as f32 / runs as f32 * 100.0;
        let avg_triggers = total_triggers as f32 / runs as f32;

        println!(
            "│ {:<31} │ {:>8.1}%    │ {:>10.2}  │ {:>10}  │",
            case.name, fp_rate, avg_triggers, max_triggers
        );
    }

    println!("└─────────────────────────────────┴──────────────┴──────────────┴──────────────┘");
    println!();
    println!("FP Rate = percentage of runs that had at least one false immigration trigger.");
    println!("Target: FP Rate should be <5% for low/standard noise and <15% for extreme noise.");
}
