# Genetic Algorithm Parameter Tuning

This document explains the default parameters chosen for the genetic algorithm
controller, how they were derived, and how users can adjust them for different
workloads.

## Overview

The genetic algorithm (GA) optimizes a *powercap percentage* that balances
energy consumption against runtime performance. The score function combines both
objectives:

```
score = energy^α × runtime^(1 − α)
```

where `α` is the `energy_preference` parameter (default 0.9, meaning energy
efficiency is strongly prioritized).

## Default Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `population_size` | 20 | Number of chromosomes evaluated per generation |
| `energy_preference` | 0.9 | Weight of energy in the score function (0–1) |
| `survival_rate` | 0.15 | Fraction of population that survives as elites |
| `mutation_rate` | 0.3 | Probability a child is mutated after crossover |
| `mutation_strength` | 0.02 | Maximum perturbation magnitude per gene |
| `power_min` | 0.1 | Minimum powercap percentage |
| `power_max` | 1.0 | Maximum powercap percentage |
| `immigration_rate` | *(disabled)* | Fraction replaced on workload shift |
| `immigration_change_threshold` | 0.15 | Minimum median relative score change to trigger |
| `immigration_robustness_threshold` | 2.0 | Required signal-to-noise ratio for trigger |
| `immigration_min_matched_scores` | 3 | Minimum comparable chromosomes for detection |
| `immigration_similarity_threshold` | 0.03 | Maximum parameter change to keep prev_score |
| `immigration_cooldown_generations` | 3 | Generations to wait between immigration events |

## How These Values Were Derived

### Methodology

Parameters were tuned using synthetic workload curves covering the full space
of realistic energy/runtime relationships:

- **Linear tradeoffs** (optimum at boundary): steep, moderate, and gentle slopes
- **Quadratic curves** (bowl-shaped, interior optimum): various optima locations
- **Sigmoid curves** (step transitions): sharp and gradual, symmetric and asymmetric
- **Mixed combinations**: quadratic energy + linear runtime, sigmoid + linear, etc.
- **Edge cases**: nearly flat energy or runtime curves

Each configuration was tested over 200 independent runs with measurement noise
(energy CV=2.5%, runtime CV=0.5%) representative of real RAPL measurements.
Convergence was defined as 15 of the last 20 iterations falling within a
noise-derived threshold of the analytically computed optimum.

### Key Findings

1. **Population size (20)**: Convergence speed is dominated by generations, not
   per-generation quality. A population of 20 gives the best iterations-to-convergence
   because it evaluates the full search space in fewer total samples while
   maintaining enough diversity. Larger populations (30–40) find the same quality
   solution but waste iterations evaluating redundant individuals.

2. **Survival rate (0.15)**: With `pop=20`, approximately 3 chromosomes survive.
   This provides strong selection pressure while retaining enough elite diversity
   to avoid premature convergence. Rates below 0.1 risk panics with small
   populations; rates above 0.2 slow convergence significantly.

3. **Mutation rate (0.3) and strength (0.02)**: Mutation has minimal impact on
   convergence speed in this problem because the initial uniform spread already
   covers the search space well, and crossover of survivors efficiently focuses
   the population. However, a moderate mutation rate with small strength helps
   fine-tune solutions near boundaries and prevents complete population collapse
   onto a single point.

4. **Immigration detection**: The paired-comparison approach using robust
   statistics (median + MAD) reliably detects workload shifts within a single
   generation. The thresholds (15% change, 2.0 robustness ratio) are tuned to
   trigger aggressively on genuine shifts while ignoring measurement noise.

### Benchmark Results (energy_preference=0.9)

```
┌─────────────────────────────────┬──────────┬──────────┬──────────┐
│ Test Case                       │ Median   │ Q1       │ Q3       │
├─────────────────────────────────┼──────────┼──────────┼──────────┤
│ Linear gentle                   │    52    │    49    │    55    │
│ Linear steep                    │    54    │    52    │    75    │
│ Linear extreme                  │    55    │    54    │    78    │
│ Quadratic low optimum           │    45    │    36    │    57    │
│ Quadratic mid optimum           │    43    │    41    │    48    │
│ Quadratic wide                  │    56    │    46    │    59    │
│ Sigmoid sharp                   │    42    │    41    │    44    │
│ Sigmoid gradual                 │    36    │    35    │    38    │
│ Sigmoid asymmetric              │    45    │    43    │    47    │
│ Quad energy + Lin runtime       │    45    │    41    │    56    │
│ Quad energy + Sig runtime       │    50    │    37    │    58    │
│ Sig energy + Lin runtime        │    34    │    33    │    35    │
│ Flat energy + steep runtime     │    20    │    19    │    32    │
│ Steep energy + flat runtime     │    54    │    52    │    57    │
└─────────────────────────────────┴──────────┴──────────┴──────────┘
Convergence rate: 100% across all test cases
```

## Adjusting Parameters for Different Use Cases

### By Energy Preference

The default parameters work well across all `energy_preference` values (0.1 to
0.9) without modification. The score function naturally adjusts the optimization
landscape; no parameter re-tuning is needed when changing energy preference.

However, for extreme preferences:

- **`energy_preference` close to 1.0** (pure energy optimization): The optimum
  tends to be at or near the minimum powercap. Convergence is fast (the initial
  population already covers this region). Consider reducing `population_size` to
  15 if latency is critical.

- **`energy_preference` close to 0.0** (pure runtime optimization): The optimum
  tends to be at or near the maximum powercap. Same reasoning as above applies.

- **`energy_preference` around 0.5** (balanced): The optimum is typically in the
  interior of the search space. The defaults are well-suited. If the curves are
  very flat near the optimum, convergence may be slightly slower because many
  configurations score similarly.

### For Consistent Workloads (No Shifts Expected)

If you know the workload will not change:

- **Disable immigration** by not setting `--immigration-rate`. This is the
  default behavior. The algorithm converges to the optimum and stays there,
  making only small adjustments due to measurement noise.

- **Reduce `population_size` to 15**: With a stable workload, fewer chromosomes
  are needed since we only need to find the optimum once.

- **Increase `survival_rate` to 0.2**: More survivors means less variation
  between generations, which provides more stable behavior after convergence
  at the cost of slightly slower initial convergence.

### For Frequently Changing Workloads

If the workload changes often (e.g., phase-based applications):

- **Enable immigration** with `--immigration-rate 1.0`. This fully replaces the
  population when a shift is detected, allowing rapid adaptation.

- **Reduce `immigration_cooldown_generations` to 1–2**: Allows faster response
  to consecutive shifts. The default of 3 prevents thrashing but may delay
  adaptation if shifts are rapid.

- **Reduce `immigration_change_threshold` to 0.1**: Makes the detector more
  sensitive to smaller workload changes. Risk: may trigger false positives with
  noisy measurements.

- **Keep `population_size` at 20**: After immigration, the algorithm needs
  enough diversity to quickly explore the new optimum. Larger populations waste
  time; smaller populations may not cover the space well enough.

### For High-Noise Environments

If measurements are noisy (e.g., shared systems, I/O-bound workloads):

- **Increase `population_size` to 25–30**: More samples per generation give
  a more reliable picture of the fitness landscape.

- **Increase `immigration_change_threshold` to 0.2–0.3**: Avoids false
  immigration triggers from noise spikes.

- **Increase `immigration_robustness_threshold` to 3.0**: Requires stronger
  statistical evidence before triggering immigration.

The convergence threshold automatically adapts to noise levels through the
`derive_score_error_threshold` function, so the algorithm remains robust without
manual intervention.

### For Very Narrow Optima

If you know the optimal powercap occupies a very narrow range:

- **Decrease `mutation_strength` to 0.01**: Smaller perturbations for finer
  exploration near the optimum.

- **Increase `mutation_rate` to 0.5**: More frequent small mutations help the
  population explore the neighborhood of survivors.

## Reproducing the Tuning Results

The `tune` workspace provides four binaries for evaluation:

```bash
# Single-run detailed output showing convergence behavior
cargo run --release --bin test -- [OPTIONS]

# Multi-run convergence statistics (from cold start)
cargo run --release --bin tune -- -i 200 [OPTIONS]

# Multi-run adaptation statistics (workload shift detection)
cargo run --release --bin adapt -- -i 200 --immigration-rate 1.0 [OPTIONS]

# Comprehensive benchmark across all curve families
cargo run --release --bin bench -- -i 200 [OPTIONS]
```

All binaries accept `--energy-curve` and `--runtime-curve` parameters in the
format `Type:param1,param2,...`:

- `Linear:lb,ub` — linear interpolation from `lb` (at powercap=0) to `ub` (at powercap=1)
- `Quadratic:lb,t_middle,steepness` — bowl curve with minimum at `t_middle`
- `Sigmoid:lb,ub,t_middle,steepness` — S-shaped transition centered at `t_middle`

Enable logging with `RUST_LOG=info` (or `debug`/`trace`) for detailed output:

```bash
RUST_LOG=info cargo run --release --bin test
RUST_LOG=debug cargo run --release --bin adapt -- --immigration-rate 1.0
```

## Architecture Notes

- The population is initialized as a uniform spread over `[power_min, power_max]`,
  not randomly. This ensures good coverage in the first generation and often
  identifies the optimal region immediately.

- Between generations, the population is sorted by powercap in alternating
  ascending/descending order. This minimizes large jumps in the applied powercap
  between consecutive iterations, reducing perturbation to the running system.

- Immigration detection uses robust paired statistics (median relative change and
  MAD-based signal-to-noise ratio) applied only to chromosomes that have
  comparable previous scores. This avoids confusing genetic variation with
  workload changes.

- Thread control is disabled by default (`--do-thread-control` flag). When
  enabled later, the chromosome gains an additional gene for thread percentage,
  and the population spreads over a 2D search space. The same parameters should
  work but may benefit from a slightly larger population (25–30) to cover the
  expanded space.
