use clap::Parser;
use controller::*;
use prelude::*;

const BENCHMARK_RUNS: usize = 500;
const MAX_ITERATIONS: usize = 200;
const CONVERGENCE_WINDOW: usize = 20;
const CONVERGENCE_REQUIRED: usize = 15;
const CONVERGENCE_THRESHOLD_MULTIPLIER: f32 = 1.5;

/// Comprehensive benchmark of the genetic algorithm across many workload shapes.
/// Outputs a summary table suitable for inclusion in documentation or papers.
#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// Coefficient of variation for energy measurements.
    #[arg(long, default_value_t = 0.025)]
    energy_cv: f32,
    #[arg(long, default_value_t = 0.005)]
    runtime_cv: f32,

    #[arg(long)]
    tikz: bool,

    #[command(flatten)]
    config: GeneticControllerConfig,
}

struct TestCase {
    name: &'static str,
    energy_curve: Curve,
    runtime_curve: Curve,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        // Linear tradeoffs (optimum at boundary)
        TestCase {
            name: "Linear",
            energy_curve: "Linear:0.2,0.5".parse().unwrap(),
            runtime_curve: "Linear:0.5,0.2".parse().unwrap(),
        },
        // Quadratic (bowl-shaped, clear interior optimum)
        TestCase {
            name: "Quadratic aligned",
            energy_curve: "Quadratic:0.1,0.4,2.0".parse().unwrap(),
            runtime_curve: "Quadratic:0.7,0.1,2.0".parse().unwrap(),
        },
        TestCase {
            name: "Quadratic shifted",
            energy_curve: "Quadratic:0.1,0.4,2.0".parse().unwrap(),
            runtime_curve: "Quadratic:0.1,0.7,2.0".parse().unwrap(),
        },
        TestCase {
            name: "Quadratic mirrored",
            energy_curve: "Quadratic:0.05,0.3,1.5".parse().unwrap(),
            runtime_curve: "Quadratic:0.05,0.7,1.5".parse().unwrap(),
        },
        // Sigmoid (step-like transitions)
        TestCase {
            name: "Sigmoid aligned",
            energy_curve: "Sigmoid:0.1,0.8,0.5,15.0".parse().unwrap(),
            runtime_curve: "Sigmoid:0.1,0.8,0.5,15.0".parse().unwrap(),
        },
        TestCase {
            name: "Sigmoid shifted",
            energy_curve: "Sigmoid:0.1,0.8,0.3,8.0".parse().unwrap(),
            runtime_curve: "Sigmoid:0.1,0.8,0.7,8.0".parse().unwrap(),
        },
        TestCase {
            name: "Sigmoid inverted",
            energy_curve: "Sigmoid:0.1,0.8,0.4,8.0".parse().unwrap(),
            runtime_curve: "Sigmoid:0.1,0.8,0.6,-8.0".parse().unwrap(),
        },
        // Mixed curve types
        TestCase {
            name: "Quad energy + Lin runtime",
            energy_curve: "Quadratic:0.1,0.2,3.0".parse().unwrap(),
            runtime_curve: "Linear:0.8,0.2".parse().unwrap(),
        },
        TestCase {
            name: "Quad energy + Sig runtime",
            energy_curve: "Quadratic:0.1,0.2,3.0".parse().unwrap(),
            runtime_curve: "Sigmoid:0.8,0.9,0.3,-100.0".parse().unwrap(),
        },
        TestCase {
            name: "Sig energy + Lin runtime",
            energy_curve: "Sigmoid:0.1,0.8,0.5,10.0".parse().unwrap(),
            runtime_curve: "Linear:0.8,0.2".parse().unwrap(),
        },
        // Edge cases (nearly flat in one dimension)
        TestCase {
            name: "Flat energy + steep runtime",
            energy_curve: "Linear:0.3,0.35".parse().unwrap(),
            runtime_curve: "Linear:0.8,0.2".parse().unwrap(),
        },
        TestCase {
            name: "Steep energy + flat runtime",
            energy_curve: "Linear:0.2,0.8".parse().unwrap(),
            runtime_curve: "Linear:0.45,0.55".parse().unwrap(),
        },
    ]
}

fn run(
    best_score: f32,
    config: &GeneticControllerConfig,
    energy_curve: Curve,
    runtime_curve: Curve,
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

        if has_converged(&recent_score_error_ratios, convergence_score_threshold) {
            return Some(iteration);
        }
    }

    None
}

fn main() {
    env_logger::init();

    let Args {
        energy_cv,
        runtime_cv,
        tikz,
        mut config,
    } = Args::parse();

    config.threads_min = 1.0;
    config.threads_max = 1.0;

    let cases = get_test_cases();

    if tikz {
        println!("\\documentclass{{paper}}");
        println!("\\usepackage{{xcolor}}");
        println!("\\definecolor{{energycolor}}{{RGB}}{{2,158,115}}");
        println!("\\definecolor{{runtimecolor}}{{RGB}}{{5,121,153}}");
        println!("\\definecolor{{escorecolor}}{{RGB}}{{222,143,5}}");
        println!("\\usepackage{{pgfplots}}");
        println!("\\usetikzlibrary{{patterns,external}}");
        println!("\\begin{{document}}");
        println!();

        for case in &cases {

            println!("{}", case.name);
            println!();

            println!("\\begin{{tikzpicture}}[scale=0.8]");
            println!("\\begin{{axis}}[");
            println!("declare function={{");

            println!("f(\\x) = {};", case.energy_curve.to_tikz());
            println!("g(\\x) = {};", case.runtime_curve.to_tikz());

            println!("score(\\x,\\alpha) = f(\\x)^\\alpha * g(\\x)^(1 - \\alpha);");

            println!("}}]");

            println!("\\addplot[domain=0:1,samples=50,color=energycolor] {{f(x)}};");
            println!("\\addplot[domain=0:1,samples=50,color=runtimecolor] {{g(x)}};");

            println!("\\addplot[domain=0:1,samples=50,color=escorecolor,dashed] {{score(x,0.1)}};");
            println!("\\addplot[domain=0:1,samples=50,color=escorecolor,dash pattern={{on 2pt off 2pt}}] {{score(x,0.5)}};");
            println!("\\addplot[domain=0:1,samples=50,color=escorecolor,dash pattern={{on 7pt off 2pt on 1pt off 2pt}}] {{score(x,0.9)}};");

            println!("\\end{{axis}}");
            println!("\\end{{tikzpicture}}");

            println!();
        }

        println!("\\end{{document}}");

        return;
    }

    let convergence_score_threshold = derive_score_error_threshold(
        config.energy_preference,
        energy_cv,
        runtime_cv,
        CONVERGENCE_THRESHOLD_MULTIPLIER,
    );

    println!("┌─────────────────────────────────┬──────────┬──────────┬──────────┬──────────┬────────┐");
    println!("│ Test Case                       │ Optimal  │ Median   │ Q1       │ Q3       │ Conv%  │");
    println!("├─────────────────────────────────┼──────────┼──────────┼──────────┼──────────┼────────┤");

    let mut pcap_min = f32::MAX;
    let mut pcap_max = f32::MIN;
    let mut median_min = usize::MAX;
    let mut median_max = usize::MIN;
    let mut q1_min = usize::MAX;
    let mut q1_max = usize::MIN;
    let mut q3_min = usize::MAX;
    let mut q3_max = usize::MIN;
    let mut conv_min = f32::MAX;
    let mut conv_max = f32::MIN;

    for case in &cases {
        let (best_score, _, _, best_powercap) = find_optimal_powercap(
            config.energy_preference,
            case.energy_curve,
            case.runtime_curve,
            config.power_min,
            config.power_max,
        );

        pcap_min = pcap_min.min(best_powercap);
        pcap_max = pcap_max.max(best_powercap);

        let mut run_results = vec![usize::MAX / 2; BENCHMARK_RUNS];
        let mut converged_count = 0;
        for i in 0..BENCHMARK_RUNS {
            let converged = run(
                best_score,
                &config,
                case.energy_curve,
                case.runtime_curve,
                energy_cv,
                runtime_cv,
                convergence_score_threshold,
            );

            if let Some(iterations) = converged {
                run_results[i] = iterations;
                converged_count += 1;
            }
        }

        let (med, q1, q3) = quartiles(run_results);
        let conv_pct = converged_count as f32 / BENCHMARK_RUNS as f32 * 100.0;

        median_min = median_min.min(med);
        median_max = median_max.max(med);
        q1_min = q1_min.min(q1);
        q1_max = q1_max.max(q1);
        q3_min = q3_min.min(q3);
        q3_max = q3_max.max(q3);
        conv_min = conv_min.min(conv_pct);
        conv_max = conv_max.max(conv_pct);

        if conv_pct > 50.0 {
            println!(
                "│ {:<31} │ {:>6.3}W  │ {:>5}    │ {:>5}    │ {:>5}    │ {:>5.1}% │",
                case.name, best_powercap, med, q1, q3, conv_pct
            );
        } else {
            println!(
                "│ {:<31} │ {:>6.3}W  │ FAIL     │ FAIL     │ FAIL     │ {:>5.1}% │",
                case.name, best_powercap, conv_pct
            );
        }
    }

    println!("├─────────────────────────────────┼──────────┼──────────┼──────────┼──────────┼────────┤");
    println!(
        "│ {:<31} │ {:>6.3}W  │ {:>5}    │ {:>5}    │ {:>5}    │ {:>5.1}% │",
        "Min", pcap_min, median_min, q1_min, q3_min, conv_min,
    );
    println!(
        "│ {:<31} │ {:>6.3}W  │ {:>5}    │ {:>5}    │ {:>5}    │ {:>5.1}% │",
        "Max", pcap_max, median_max, q1_max, q3_max, conv_max,
    );

    println!("└─────────────────────────────────┴──────────┴──────────┴──────────┴──────────┴────────┘");
    println!();
    println!("Configuration: pop={}, sr={}, mr={}, ms={}, e_pref={}",
        config.population_size, config.survival_rate, config.mutation_rate,
        config.mutation_strength, config.energy_preference);
    println!("Convergence: {}/{} within {:.2}% of optimal over {} runs (max {} iters)",
        CONVERGENCE_REQUIRED, CONVERGENCE_WINDOW, convergence_score_threshold * 100.0, BENCHMARK_RUNS, MAX_ITERATIONS);
}

fn has_converged(recent_score_error_ratios: &[f32], convergence_score_threshold: f32) -> bool {
    debug_assert_eq!(recent_score_error_ratios.len(), CONVERGENCE_WINDOW);
    let num_converged = recent_score_error_ratios
        .iter()
        .filter(|&&score_error_ratio| score_error_ratio <= convergence_score_threshold)
        .count();
    num_converged >= CONVERGENCE_REQUIRED
}
