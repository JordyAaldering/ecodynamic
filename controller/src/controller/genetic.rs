use clap::Parser;

use crate::{Capabilities, Controller, Demand, Sample, filter_functions::median, scores};

pub struct GeneticController {
    samples: Vec<Sample>,
    population: Vec<Chromosome>,
    immigration_cooldown: usize,
    sort_descending: bool,
    max_threads: u16,
    effective_mutation_rate: f32,
    config: GeneticControllerConfig,
    // Debugging metadata
    generation: usize,
    immigration_was_triggered: bool,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct GeneticControllerConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub population_size: usize,

    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while a value of 0 means that only runtime performance is optimised for.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.9)]
    pub energy_preference: f32,

    /// Minimum allowed percentage of the number of threads.
    ///
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.1)]
    pub threads_min: f32,

    /// Maximum allowed percentage of the number of threads.
    ///
    /// Range: (0,1].
    #[arg(long, default_value_t = 1.0)]
    pub threads_max: f32,

    /// Minimum allowed percentage of the powercap.
    ///
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.1)]
    pub power_min: f32,

    /// Maximum allowed percentage of the powercap.
    ///
    /// Range: (0,1].
    #[arg(long, default_value_t = 1.0)]
    pub power_max: f32,

    /// By default, the first chromosomes will have low thread counts and power limits,
    /// and the last chromosomes will have high thread counts and power limits.
    /// Setting this value to true reverses this order.
    #[arg(long)]
    pub initial_population_descending: bool,

    /// Genetic algorithm survival rate. Controls the fraction of the population that
    /// survives into the next generation as elite individuals.
    ///
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.15)]
    pub survival_rate: f32,

    /// Mutation strength: maximum magnitude of a random perturbation applied to each gene.
    ///
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.01)]
    pub mutation_strength: f32,

    /// Mutation rate: probability that a child chromosome is mutated after crossover.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.3)]
    pub mutation_rate: f32,

    /// Mutation rate decay factor. After each generation, the effective mutation rate is
    /// multiplied by this factor, decaying toward a minimum. This allows aggressive exploration
    /// early on and fine-tuning as the population converges. Set to 1.0 to disable decay.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.7)]
    pub mutation_rate_decay: f32,

    /// Minimum mutation rate after decay. The effective mutation rate will never drop below this.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.01)]
    pub mutation_rate_min: f32,

    /// Immigration can result in very poor chromosomes and might thus be very costly. We want to
    /// avoid immigration to occur in every evolution step. Setting the value to less than
    /// 1 / population_size ensures this.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 1.0)]
    pub immigration_rate: f32,

    /// Minimum median relative score change required to trigger immigration.
    #[arg(long, default_value_t = 0.075)]
    pub immigration_change_threshold: f32,

    /// Minimum robust z-like score required to trigger immigration.
    /// This is the ratio of median change to MAD (median absolute deviation).
    /// Higher values require more consistent shifts across chromosomes, filtering
    /// out random noise while still detecting genuine workload changes.
    #[arg(long, default_value_t = 10.0)]
    pub immigration_robustness_threshold: f32,

    /// Minimum number of comparable chromosomes needed before trigger detection is active.
    ///
    /// Although it may seem unlikely that 5 of the 20 chromosomes have similar scores, as
    /// chromosomes initially spread the configuration space, note that as the controller
    /// converges, chromosomes will start to cluster together, resulting in similar scores.
    /// Thus, this happens to produce exactly the behaviour we want: before convergence immigration
    /// is unlikely to trigger, which is good because we have no way of telling whether score
    /// changes are due to workload shifts or just the exploration process. After convergence
    /// chromosomes are more similar, and score changes are more likely to be due to workload shifts.
    #[arg(long, default_value_t = 5)]
    pub immigration_min_matched_scores: usize,

    /// Maximum allowed per-parameter change when reusing a previous score.
    #[arg(long, default_value_t = 0.03)]
    pub immigration_similarity_threshold: f32,

    /// Number of generations to wait before allowing immigration to trigger again.
    #[arg(long, default_value_t = 3)]
    pub immigration_cooldown_generations: usize,
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}

impl GeneticController {
    /// Instead of randomly initialized values, use an even spread over valid thread
    /// counts and power limits to reduce duplication and increase the chances of
    /// finding an optimum immediately.
    pub fn new(config: GeneticControllerConfig, caps: &Capabilities) -> Self {
        let population = (0..config.population_size)
            .map(|mut i| {
                if config.initial_population_descending {
                    i = config.population_size - i - 1;
                }

                let t = i as f32 / (config.population_size - 1) as f32;
                let threads_pct = lerp(config.threads_min, config.threads_max, t);
                let power_pct = lerp(config.power_min, config.power_max, t);
                Chromosome::new(threads_pct, power_pct)
            })
            .collect();

        log::trace!("Init: {:?}", population);

        Self {
            samples: Vec::with_capacity(config.population_size),
            population,
            immigration_cooldown: 0,
            sort_descending: !config.initial_population_descending,
            max_threads: caps.max_threads.unwrap_or(1),
            effective_mutation_rate: config.mutation_rate,
            config,
            generation: 0,
            immigration_was_triggered: false,
        }
    }

    /// Returns the current generation number.
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Returns whether immigration was triggered during the most recent evolution.
    /// This flag is reset at the start of each evolve() call.
    pub fn immigration_triggered(&self) -> bool {
        self.immigration_was_triggered
    }
}

impl Controller for GeneticController {
    /// Use the number of samples to determine the current index into the population.
    /// The population is reset every `population_size` iterations.
    /// In between, we want every chromosome to be applied once.
    fn get_demand(&self) -> Demand {
        debug_assert!(self.samples.len() < self.population.len());
        let chromosome = &self.population[self.samples.len()];
        Demand {
            powercap_pct: chromosome.power_pct,
            num_threads: ((chromosome.threads_pct * self.max_threads as f32).round() as u16).max(1),
        }
    }

    fn push_sample(&mut self, sample: Sample) {
        self.samples.push(sample);

        if self.samples.len() >= self.config.population_size {
            self.evolve();
            self.samples.clear();
        }
    }
}

impl GeneticController {
    fn evolve(&mut self) {
        self.generation += 1;
        self.immigration_was_triggered = false;

        let GeneticControllerConfig {
            energy_preference,
            survival_rate,
            immigration_rate,
            immigration_change_threshold,
            immigration_cooldown_generations,
            immigration_min_matched_scores,
            immigration_robustness_threshold,
            mutation_rate_decay,
            mutation_rate_min,
            ..
        } = self.config;

        let scores = scores(&self.samples, energy_preference);

        log::debug!("Generation {}: best_score={:.4}, worst_score={:.4}, median_score={:.4}",
            self.generation,
            scores.iter().cloned().reduce(f32::min).unwrap_or(0.0),
            scores.iter().cloned().reduce(f32::max).unwrap_or(0.0),
            {
                let mut s = scores.clone();
                s.sort_by(f32::total_cmp);
                s[s.len() / 2]
            }
        );

        let change_detected = update_prev_scores_and_check_for_shift(
            &mut self.population,
            &scores,
            immigration_change_threshold,
            immigration_robustness_threshold,
            immigration_min_matched_scores,
        );

        let population_size = self.population.len();

        // Ensure at least 1 survivor to avoid empty range panic in crossover selection.
        let survival_count = {
            let survival_count = population_size as f32 * survival_rate;
            let survival_remainder = survival_count.fract();
            let mut survival_count = survival_count.floor() as usize;
            if rand::random_bool(survival_remainder as f64) {
                survival_count += 1;
            }
            survival_count.max(1)
        };

        let immigration_start = {
            let do_immigration = if self.immigration_cooldown > 0 {
                self.immigration_cooldown -= 1;
                false
            } else if change_detected {
                self.immigration_cooldown = immigration_cooldown_generations;
                true
            } else {
                false
            };

            if do_immigration {
                self.immigration_was_triggered = true;
                log::info!("Generation {}: immigration triggered, replacing population with spread individuals", self.generation);

                // Reset mutation rate on immigration to allow aggressive exploration of new landscape
                self.effective_mutation_rate = self.config.mutation_rate;

                // When immigration rate is less than 1 / population_size, we use a random
                // chance based on the remainder to ensure immigration can still occur.
                let immigration_count = population_size as f32 * immigration_rate;
                let immigration_remainder = immigration_count.fract();
                let mut immigration_count = immigration_count.floor() as usize;
                if rand::random_bool(immigration_remainder as f64) {
                    immigration_count += 1;
                }

                // If survival_rate + immigration_rate > 1.0, there is some overlap between the two.
                // We decide to favor immigration over survival, meaning that fewer than survival_count chromosomes may survive.
                // To favor survival instead, max by survival_count instead of 0.
                (population_size - immigration_count).max(0)
            } else {
                population_size
            }
        };

        sort_population_by_score(&mut self.population, scores);

        // Replace chromosomes by children of the best performing chromosomes
        let effective_mr = self.effective_mutation_rate;
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(parent2, &self.config);
            if rand::random_bool(effective_mr as f64) {
                child.mutate(&self.config);
            }

            self.population[i] = child;
        }

        // Decay the mutation rate for next generation
        self.effective_mutation_rate = (self.effective_mutation_rate * mutation_rate_decay).max(mutation_rate_min);
        log::debug!("Generation {}: effective_mutation_rate={:.4}",
            self.generation, self.effective_mutation_rate);

        // Fill remaining chromosomes by immigration
        let immigration_count = population_size.saturating_sub(immigration_start);
        for (offset, i) in (immigration_start..population_size).enumerate() {
            self.population[i] = Chromosome::from_spread(offset, immigration_count, &self.config);
        }

        // To minimise changes in the runtime we sort by the recommended power limit
        // and we oscillate between an increasing and decreasing order.
        if self.sort_descending {
            self.population.sort_by(|a, b| b.partial_cmp(a).unwrap());
        } else {
            self.population.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        self.sort_descending = !self.sort_descending;
        log::trace!("Evolve: {:?}", self.population);
    }
}

fn sort_population_by_score(population: &mut Vec<Chromosome>, scores: Vec<f32>) {
    let mut combined: Vec<_> = population.drain(..).zip(scores).collect();
    combined.sort_unstable_by(|(_, a), (_, b)| a.total_cmp(b));
    *population = combined.into_iter().map(|(c, _)| c).collect();
}

/// Detect whether program behaviour appears to have shifted between the previous
/// and current generation. The approach is grounded in robust statistics: the
/// median and MAD replace mean and standard deviation, and the ratio `median_delta / MAD`
/// is a robust signal-to-noise measure equivalent to the modified z-score. The paired-comparison
/// structure is the non-parametric analogue of a paired t-test.
///
/// We only compare chromosomes that still have a `prev_score`, which means they
/// are considered similar enough to their earlier version that the score
/// comparison is meaningful. Freshly immigrated chromosomes and crossover
/// children do not carry history, and mutations clear history once the parameter
/// change is large enough. This avoids confusing workload changes with score
/// differences caused by evaluating completely different configurations.
///
/// For each comparable chromosome, we compute the absolute relative change
/// between the old and new score. We then summarize those paired changes with
/// the median, because energy and runtime measurements can be noisy and a single
/// outlier should not force immigration on its own. The median must exceed a
/// minimum change threshold before we consider the shift meaningful.
///
/// We additionally compute the median absolute deviation (MAD) around that
/// median change. This gives a cheap robust measure of how consistent the shift
/// is across the comparable chromosomes. Immigration is only triggered when the
/// median change is both large enough in absolute terms and large relative to
/// that spread.
///
/// This design intentionally uses only the previous and current generation. The
/// goal is to answer a local question: did the workload appear to change since
/// the last time these comparable configurations were observed? That keeps the
/// logic simple, avoids detector state that must be carried across many
/// generations, and fits the genetic algorithm better than treating all samples
/// as one continuous stationary process.
fn update_prev_scores_and_check_for_shift(
    population: &mut [Chromosome],
    scores: &[f32],
    change_threshold: f32,
    robustness_threshold: f32,
    min_matched_scores: usize,
) -> bool {
    debug_assert_eq!(population.len(), scores.len());

    let mut deltas = Vec::with_capacity(scores.len());

    for (chromosome, &score) in population.iter_mut().zip(scores.iter()) {
        if let Some(prev_score) = chromosome.prev_score {
            // Use a relative change metric to normalize across regions with different absolute scales.
            let ratio = score / (prev_score + f32::EPSILON);
            deltas.push((ratio - 1.0).abs());
        }
        chromosome.prev_score = Some(score);
    }

    if deltas.len() < min_matched_scores {
        log::debug!("Shift detection: {} comparable chromosomes (need {})", deltas.len(), min_matched_scores);
        return false;
    }

    log::debug!("Shift detection: {} comparable chromosomes", deltas.len());

    let median_delta = median(&mut deltas);
    log::trace!("Shift detection: median_delta={:.4}, threshold={:.4}", median_delta, change_threshold);
    if median_delta < change_threshold {
        return false;
    }

    let mut deviations: Vec<_> = deltas.into_iter().map(|delta| (delta - median_delta).abs()).collect();
    let mad = median(&mut deviations);
    let ratio = median_delta / (mad + f32::EPSILON);
    if ratio >= robustness_threshold {
        log::info!("Shift detected: median change = {:.2}%, robustness ratio = {:.2}", median_delta * 100.0, ratio);
        true
    } else {
        false
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chromosome {
    threads_pct: f32,
    power_pct: f32,
    prev_score: Option<f32>,
}

impl Chromosome {
    fn new(threads_pct: f32, power_pct: f32) -> Self {
        Self {
            threads_pct,
            power_pct,
            prev_score: None,
        }
    }

    /// Generate a random chromosome for immigration
    fn rand(config: &GeneticControllerConfig) -> Self {
        let num_threads = rand::random_range(0.1..=1.0);
        let power_limit_pct = rand::random_range(config.power_min..=config.power_max);
        Self::new(num_threads, power_limit_pct)
    }

    /// Generate a chromosome using an even spread over the valid search space.
    ///
    /// This includes exact min and max values whenever `count >= 2`.
    fn from_spread(index: usize, count: usize, config: &GeneticControllerConfig) -> Self {
        debug_assert_ne!(count, 0);
        if count == 1 {
            return Chromosome::rand(config);
        }

        let t = index as f32 / (count - 1) as f32;
        let threads_pct = lerp(config.threads_min, config.threads_max, t);
        let power_pct = lerp(config.power_min, config.power_max, t);
        Self::new(threads_pct, power_pct)
    }

    fn crossover(&self, other: &Self, config: &GeneticControllerConfig) -> Self {
        // Use the same ratio for the thread count and power limit, as typically lowering
        // the number of threads enables a reduction in the power limit, and vice versa.
        let t = rand::random_range(0.0..=1.0);

        // If both parents are similar (similar thread count and power limit), we use their
        // scores to instantiate the child score. This is needed for immigration detection to
        // work, as it relies on comparing the previous and current scores of similar chromosomes.
        // With the default survival rate of only 15%, not enough prev_scores would survive each
        // generation to result in enough scores to detect a workload shift. By averaging the
        // scores of similar parents, we can ensure that the child has a prev_score that is
        // hopefully representative of its configuration.
        let prev_score = if self.is_similar_to(other, config.immigration_similarity_threshold) {
            match (self.prev_score, other.prev_score) {
                (Some(left), Some(right)) => Some((left + right) * t),
                _ => None,
            }
        } else {
            None
        };

        Self {
            threads_pct: (self.threads_pct + other.threads_pct) * t,
            power_pct: (self.power_pct + other.power_pct) * t,
            prev_score,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, config: &GeneticControllerConfig) {
        let prev_threads_pct = self.threads_pct;
        let prev_power_pct = self.power_pct;

        self.threads_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.threads_pct = self.threads_pct.max(0.1).min(1.0);

        self.power_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.power_pct = self.power_pct.max(config.power_min).min(config.power_max);

        if (self.threads_pct - prev_threads_pct).abs() > config.immigration_similarity_threshold
            || (self.power_pct - prev_power_pct).abs() > config.immigration_similarity_threshold {
            self.prev_score = None;
        }
    }

    fn is_similar_to(&self, other: &Self, similarity_threshold: f32) -> bool {
        (self.threads_pct - other.threads_pct).abs() <= similarity_threshold
            && (self.power_pct - other.power_pct).abs() <= similarity_threshold
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.power_pct.partial_cmp(&other.power_pct)
    }
}
