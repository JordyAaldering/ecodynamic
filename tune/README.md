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
| `mutation_rate` | 0.3 | Initial mutation probability after crossover |
| `mutation_rate_decay` | 0.9 | Decay factor applied to mutation rate per generation |
| `mutation_rate_min` | 0.1 | Minimum mutation rate after decay |
| `mutation_strength` | 0.02 | Maximum perturbation magnitude per gene |
| `power_min` | 0.1 | Minimum powercap percentage |
| `power_max` | 1.0 | Maximum powercap percentage |
| `immigration_rate` | 1.0 | Fraction replaced on workload shift (full replacement) |
| `immigration_change_threshold` | 0.03 | Minimum median relative score change to trigger |
| `immigration_robustness_threshold` | 8.0 | Required signal-to-noise ratio for trigger |
| `immigration_min_matched_scores` | 5 | Minimum comparable chromosomes for detection |
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

Each configuration was tested over 100+ independent runs with measurement noise
(energy CV=2.5%, runtime CV=0.5%) representative of real RAPL measurements.
Convergence was defined as 15 of the last 20 iterations falling within a
noise-derived threshold of the analytically computed optimum.

Additional benchmarks validated:
- **Stability** (false positive test): Runs under a stable workload with varying
  noise levels (CV=2.5%–15%) to verify immigration does NOT trigger falsely.
- **Drift tracking**: Monotonically shifting workloads to verify the GA tracks
  gradual changes via mutations without needing immigration.
- **Adaptation**: Abrupt workload shifts to verify immigration triggers correctly
  and the GA re-converges quickly.

### Key Findings

1. **Population size (20)**: Convergence speed is dominated by generations, not
   per-generation quality. A population of 20 gives the best iterations-to-convergence
   because it evaluates the full search space in fewer total samples while
   maintaining enough diversity. After the first generation (which has no
   comparable chromosomes), generation 2 typically has 7–9 comparable chromosomes,
   and from generation 3 onward all 20 are comparable — providing robust shift
   detection.

2. **Survival rate (0.15)**: With `pop=20`, approximately 3 chromosomes survive.
   This provides strong selection pressure while retaining enough elite diversity
   to avoid premature convergence. Rates below 0.1 risk panics with small
   populations; rates above 0.2 slow convergence significantly.

3. **Mutation rate (0.3) with decay (0.9, min 0.1)**: The initial mutation rate
   of 0.3 provides aggressive exploration during early convergence. The decay
   factor of 0.9 gradually reduces the effective rate after each generation,
   reducing noise once the population has converged. The minimum of 0.1 ensures
   at least 2 mutations per generation always occur, which is critical for
   tracking gradual workload drift. On immigration, the mutation rate resets
   to the initial value for fresh exploration.

4. **Mutation strength (0.02)**: Small perturbations allow fine-tuning near
   the optimum while being large enough to meaningfully explore near boundaries.
   Combined with the decaying mutation rate, this provides fine-grained
   adjustment after convergence.

5. **Immigration detection (change=0.03, robustness=8.0)**: The paired-comparison
   approach uses a low absolute threshold (3% change) combined with a high
   robustness requirement (ratio ≥ 8.0). This works because:
   - **Real shifts** affect all chromosomes uniformly → low MAD → high ratio
   - **Random noise** affects chromosomes inconsistently → high MAD → low ratio
   - The combination achieves <5% false positive rate even under extreme noise
     (CV=15%), while still detecting shifts as small as 4% in score.

### Benchmark Results (energy_preference=0.9)

**Cold-start convergence** (100 runs per case, max 200 iterations):
```
┌─────────────────────────────────┬──────────┬──────────┬──────────┬──────────┐
│ Test Case                       │ Median   │ Q1       │ Q3       │ Conv%    │
├─────────────────────────────────┼──────────┼──────────┼──────────┼──────────┤
│ Linear gentle                   │    52    │    50    │    55    │   99%    │
│ Linear steep                    │    53    │    51    │    66    │   99%    │
│ Linear extreme                  │    55    │    53    │    77    │  100%    │
│ Quadratic low optimum           │    53    │    40    │    58    │  100%    │
│ Quadratic mid optimum           │    43    │    41    │    54    │  100%    │
│ Quadratic wide                  │    55    │    42    │    59    │  100%    │
│ Sigmoid sharp                   │    42    │    38    │    44    │  100%    │
│ Sigmoid gradual                 │    36    │    35    │    38    │  100%    │
│ Sigmoid asymmetric              │    45    │    43    │    48    │  100%    │
│ Quad energy + Lin runtime       │    46    │    42    │    58    │  100%    │
│ Quad energy + Sig runtime       │    49    │    42    │    58    │  100%    │
│ Sig energy + Lin runtime        │    34    │    32    │    35    │  100%    │
│ Flat energy + steep runtime     │    20    │    19    │    30    │  100%    │
│ Steep energy + flat runtime     │    55    │    53    │    78    │   99%    │
└─────────────────────────────────┴──────────┴──────────┴──────────┴──────────┘
```

**Stability (false positive immigration)** — no workload change, 500 iterations per run:
```
┌─────────────────────────────────┬──────────────┐
│ Test Case                       │ FP Rate      │
├─────────────────────────────────┼──────────────┤
│ Quadratic (low noise, CV=2.5%)  │      4%      │
│ Sigmoid (low noise, CV=2.5%)    │      3%      │
│ Linear (low noise, CV=2.5%)     │      4%      │
│ Quadratic (high noise, CV=8%)   │      5%      │
│ Sigmoid (high noise, CV=8%)     │      5%      │
│ Linear (high noise, CV=8%)      │      4%      │
│ Quadratic (extreme, CV=15%)     │      2%      │
│ Linear (extreme, CV=15%)        │      3%      │
└─────────────────────────────────┴──────────────┘
```

**Monotonic drift tracking** — gradual workload shift over 1000 iterations:
```
┌─────────────────────────────────┬──────────────┬──────────────┐
│ Test Case                       │ Avg Track %  │ Immigr. Rate │
├─────────────────────────────────┼──────────────┼──────────────┤
│ Quadratic slow drift (+0.1)     │     3.8%     │      4%      │
│ Quadratic moderate drift (+0.2) │    11.9%     │      4%      │
│ Quadratic fast drift (+0.4)     │    58.0%     │     46%      │
│ Sigmoid slow drift (+0.1)       │     1.8%     │      2%      │
│ Sigmoid moderate drift (+0.2)   │     1.8%     │      2%      │
│ Sigmoid fast drift (+0.4)       │     1.8%     │      2%      │
└─────────────────────────────────┴──────────────┴──────────────┘
```

Note: Fast quadratic drift (+0.4 over 1000 iterations) legitimately triggers
immigration because the optimum moves too quickly for mutations alone to track.
This is correct behavior — immigration provides the necessary reset.

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

- **Reduce `immigration_rate` to 0** or set `--immigration-change-threshold` to
  1.0 (effectively disabling shift detection). The algorithm converges to the
  optimum and stays there, making only small adjustments due to measurement noise.

- **Reduce `population_size` to 15**: With a stable workload, fewer chromosomes
  are needed since we only need to find the optimum once.

- **Increase `survival_rate` to 0.2**: More survivors means less variation
  between generations, which provides more stable behavior after convergence
  at the cost of slightly slower initial convergence.

- **Decrease `mutation_rate_min` to 0.05 or lower**: Less ongoing mutation is
  needed since there's no drift to track.

### For Frequently Changing Workloads

If the workload changes often (e.g., phase-based applications):

- **Keep `immigration_rate` at 1.0**: Full replacement ensures the population
  can explore the new landscape without being anchored to the old optimum.

- **Reduce `immigration_cooldown_generations` to 1–2**: Allows faster response
  to consecutive shifts. The default of 3 prevents thrashing but may delay
  adaptation if shifts are rapid.

- **Reduce `immigration_robustness_threshold` to 5.0–6.0**: Makes the detector
  more sensitive to workload changes. Risk: slightly more false positives under
  high noise.

- **Keep `population_size` at 20**: After immigration, the algorithm needs
  enough diversity to quickly explore the new optimum. Larger populations waste
  time; smaller populations may not cover the space well enough.

### For Gradually Drifting Workloads

If the workload shifts slowly over time (e.g., increasing input sizes):

- **Increase `mutation_rate_min` to 0.15–0.2**: Ensures enough ongoing mutations
  to track the drift without needing immigration.

- **Set `mutation_rate_decay` to 0.95 or 1.0**: Slower (or no) decay keeps
  mutation pressure high, allowing continuous adaptation.

- **Increase `mutation_strength` to 0.03–0.05**: Larger perturbations allow
  faster tracking of the moving optimum.

- These settings trade convergence precision for tracking ability. The
  population will oscillate more around the optimum but will follow it as
  it moves.

### For High-Noise Environments

If measurements are noisy (e.g., shared systems, I/O-bound workloads):

- **Increase `population_size` to 25–30**: More samples per generation give
  a more reliable picture of the fitness landscape.

- **Increase `immigration_robustness_threshold` to 10.0–12.0**: Requires
  stronger statistical evidence before triggering immigration.

- **Increase `immigration_min_matched_scores` to 8–10**: Requires more
  comparable data points before making a shift decision.

The convergence threshold automatically adapts to noise levels through the
`derive_score_error_threshold` function, so the algorithm remains robust without
manual intervention.

### For Very Narrow Optima

If you know the optimal powercap occupies a very narrow range:

- **Decrease `mutation_strength` to 0.01**: Smaller perturbations for finer
  exploration near the optimum.

- **Increase `mutation_rate` to 0.5**: More frequent small mutations help the
  population explore the neighborhood of survivors.

- **Decrease `mutation_rate_decay` to 0.8**: Faster decay to reduce noise around
  the narrow optimum once found.

## Reproducing the Tuning Results

The `tune` workspace provides six binaries for evaluation:

```bash
# Single-run detailed output showing convergence behavior
cargo run --release --bin test -- [OPTIONS]

# Multi-run convergence statistics (from cold start)
cargo run --release --bin tune -- -i 200 [OPTIONS]

# Multi-run adaptation statistics (workload shift detection)
cargo run --release --bin adapt -- -i 200 [OPTIONS]

# Comprehensive benchmark across all curve families
cargo run --release --bin bench -- -i 200 [OPTIONS]

# Stability benchmark (false positive immigration test)
cargo run --release --bin bench-stability -- -i 200 [OPTIONS]

# Monotonic drift tracking benchmark
cargo run --release --bin bench-drift -- -i 100 [OPTIONS]
```

All binaries accept `--energy-curve` and `--runtime-curve` parameters in the
format `Type:param1,param2,...`:

- `Linear:lb,ub` — linear interpolation from `lb` (at powercap=0) to `ub` (at powercap=1)
- `Quadratic:lb,t_middle,steepness` — bowl curve with minimum at `t_middle`
- `Sigmoid:lb,ub,t_middle,steepness` — S-shaped transition centered at `t_middle`

Enable logging with `RUST_LOG=info` (or `debug`/`trace`) for detailed output:

```bash
RUST_LOG=info cargo run --release --bin test
RUST_LOG=debug cargo run --release --bin adapt
RUST_LOG=trace cargo run --release --bin bench-drift  # shows shift detection values
```

## Architecture Notes

- The population is initialized as a uniform spread over `[power_min, power_max]`,
  not randomly. This ensures good coverage in the first generation and often
  identifies the optimal region immediately.

- Between generations, the population is sorted by powercap in alternating
  ascending/descending order. This minimizes large jumps in the applied powercap
  between consecutive iterations, reducing perturbation to the running system.

- **Mutation rate decay**: The effective mutation rate starts at `mutation_rate`
  and decays by `mutation_rate_decay` each generation, down to `mutation_rate_min`.
  On immigration, the rate resets to the initial value. This provides aggressive
  exploration during initial convergence and after workload shifts, while
  maintaining lower but non-zero mutation for drift tracking once converged.

- **Immigration detection** uses robust paired statistics (median relative change
  and MAD-based signal-to-noise ratio) applied only to chromosomes that have
  comparable previous scores. The key principle: real workload shifts affect all
  chromosomes uniformly (low MAD, high ratio), while noise affects them
  inconsistently (high MAD, low ratio). This allows a low absolute threshold
  (3%) with high robustness (8.0) to simultaneously catch small real shifts and
  reject large noise fluctuations.

- Thread control is disabled by default (`--do-thread-control` flag). When
  enabled later, the chromosome gains an additional gene for thread percentage,
  and the population spreads over a 2D search space. The same parameters should
  work but may benefit from a slightly larger population (25–30) to cover the
  expanded space.
