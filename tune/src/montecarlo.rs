use clap::Parser;
use controller::*;
use prelude::*;

const BENCHMARK_REPEATS: usize = 500;
const MAX_ITERATIONS: usize = 300;
const CONVERGENCE_WINDOW: usize = 20;
const CONVERGENCE_REQUIRED: usize = 15;
const CONVERGENCE_THRESHOLD_MULTIPLIER: f32 = 1.5;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(long, default_value_t = 100)]
    runs: usize,

    /// Measurement coefficient of variation.
    #[arg(long, default_value_t = 0.025)]
    energy_cv: f32,
    #[arg(long, default_value_t = 0.005)]
    runtime_cv: f32,

    #[command(flatten)]
    config: GeneticControllerConfig,
}

fn run(
    best_score: f32,
    config: &GeneticControllerConfig,
    energy_curve: Curve,
    runtime_curve: Curve,
    energy_cv: f32,
    runtime_cv: f32,
    convergence_score_threshold: f32,
) -> Option<(f32, usize)> {
    let mut controller = GeneticController::new(config.clone(), &Capabilities::default());
    let mut recent_score_error_ratios = vec![f32::INFINITY; CONVERGENCE_WINDOW];
    let mut recent_score_error_index = 0;

    for i in 1..=MAX_ITERATIONS {
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

        if has_converged(&recent_score_error_ratios, convergence_score_threshold) {
            return Some((score, i));
        }
    }

    None
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

    let convergence_score_threshold = derive_score_error_threshold(
        config.energy_preference,
        energy_cv,
        runtime_cv,
        CONVERGENCE_THRESHOLD_MULTIPLIER,
    );

    for i in 0..runs {
        let energy_curve = Curve::random();
        let runtime_curve = Curve::random();

        if i < 5 {
            eprintln!("f(\\x) = {};", energy_curve.to_tikz());
            eprintln!("g(\\x) = {};", runtime_curve.to_tikz());
        }

        let (best_score, _, _, best_powercap) = find_optimal_powercap(
            config.energy_preference,
            energy_curve,
            runtime_curve,
            config.power_min,
            config.power_max,
        );

        let mut run_scores = Vec::new();
        let mut run_iterations = Vec::new();

        let mut converged_count = 0;
        for _ in 0..BENCHMARK_REPEATS {
            let converged = run(
                best_score,
                &config,
                energy_curve,
                runtime_curve,
                energy_cv,
                runtime_cv,
                convergence_score_threshold,
            );

            if let Some((final_score, iterations)) = converged {
                run_scores.push(final_score);
                run_iterations.push(iterations);
                converged_count += 1;
            }
        }

        let (final_score, _, _) = quartilesf32(run_scores);
        let (med, q1, q3) = quartiles(run_iterations);
        let conv_pct = converged_count as f32 / BENCHMARK_REPEATS as f32 * 100.0;
        println!("\"{}\",\"{}\",{},{},{},{},{},{},{:.1}", energy_curve.to_string(), runtime_curve.to_string(), best_powercap, best_score, final_score, med, q1, q3, conv_pct);
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
