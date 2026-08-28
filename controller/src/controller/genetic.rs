mod builder;
mod chromosome;
mod gene;

use clap::Parser;

use chromosome::Chromosome;

use crate::{controller::genetic::chromosome::ChromosomeConfig, *};

pub use builder::GeneticControllerBuilder;

pub struct GeneticController {
    samples: Vec<Sample>,
    population: Vec<Chromosome>,
    immigration_cooldown: usize,
    sort_descending: bool,
    effective_survival_rate: f32,
    effective_mutation_rate: f32,
    config: GeneticConfig,
    bounds: ChromosomeConfig,
    // Debugging metadata
    pub generation: usize,
    pub immigration_was_triggered: bool,
}

#[derive(Clone, Debug, Parser)]
pub struct GeneticConfig {
    #[arg(short('s'), long, default_value_t = 20)]
    pub population_size: usize,

    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while a value of 0 means that only runtime performance is optimised for.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.9)]
    pub energy_preference: f32,

    /// Enable nudging of chromosomes towards secondary preferences, such as core sharing and power limit proportional to energy preference.
    #[arg(long("nudge"))]
    pub do_nudging: bool,
    /// Upper bound on the influence secondary preferences ("nudges") may have
    /// on a chromosome's score, expressed as a fraction of that score.
    ///
    /// Nudges are applied multiplicatively rather than as a fixed offset, so they can only ever
    /// re-order chromosomes whose raw scores already fall within this fraction of one another;
    /// they can never override a clearly better scoring chromosome.
    /// The effective nudge is additionally capped by [GeneticConfig::nudge_relative_cap] so a fixed
    /// percentage doesn't dominate on flat score landscapes or become irrelevant on steep ones.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.05)]
    pub nudge_strength: f32,
    /// Caps the effective nudge strength to this fraction of the current generation's own
    /// relative score spread. This keeps nudges proportional to how much the scores actually
    /// vary this generation, rather than always applying the full [GeneticConfig::nudge_strength].
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.5)]
    pub nudge_relative_cap: f32,

    /// By default, the first chromosomes will have low thread counts and power limits,
    /// and the last chromosomes will have high thread counts and power limits.
    /// Setting this value to true reverses this order.
    #[arg(long)]
    pub initial_population_descending: bool,

    /// Genetic algorithm survival rate. Controls the fraction of the population that
    /// survives into the next generation as elite individuals.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.15)]
    pub survival_rate: f32,
    /// Survival rate decay factor. After each generation, the effective survival rate
    /// is multiplied by this factor. A decay of 0 means that no decay occurs. A decay
    /// of 0.5 means that after every generation, the effective survival rate is halved.
    /// This allows faster convergence when there is a wide range of good configurations,
    /// which would otherwise cause high variability in the selected chromosomes.
    ///
    /// No survival rate decay happens by default.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.0)]
    pub survival_rate_decay: f32,

    /// Mutation strength: maximum magnitude of a random perturbation applied to each gene.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.01)]
    pub mutation_strength: f32,
    /// Mutation rate: probability that a child chromosome is mutated after crossover.
    ///
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.3)]
    pub mutation_rate: f32,
    /// Mutation rate decay factor. After each generation, the effective mutation rate
    /// is multiplied by this factor. A decay of 0 means that no decay occurs. A decay
    /// of 0.5 means that after every generation, the effective mutation rate is halved.
    /// This allows aggressive exploration early on and fine-tuning as the population converges.
    ///
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.3)]
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

impl Controller for GeneticController {
    /// Use the number of samples to determine the current index into the population.
    /// The population is reset every `population_size` iterations.
    /// In between, we want every chromosome to be applied once.
    fn get_demand(&self) -> Demand {
        let chromosome = &self.population[self.samples.len()];
        chromosome.get_demand(&self.bounds)
    }

    fn push_sample(&mut self, sample: Sample) {
        // Store the global thread count at the time this sample was taken, so we can use it to compute alignment later.
        // Before calling push_sample, the server has already subtracted this chromosome's thread count from the global count.
        self.population[self.samples.len()].global_thread_count = Some(STATE.thread_utilization());

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

        let GeneticConfig {
            energy_preference,
            survival_rate,
            survival_rate_decay,
            immigration_rate,
            immigration_change_threshold,
            immigration_cooldown_generations,
            immigration_min_matched_scores,
            immigration_robustness_threshold,
            mutation_rate,
            mutation_rate_decay,
            mutation_rate_min,
            ..
        } = self.config;

        let scores = score(&self.samples, energy_preference);

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
            let survival_count = population_size as f32 * self.effective_survival_rate;
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

                self.effective_survival_rate = survival_rate;
                self.effective_mutation_rate = mutation_rate;

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

        if self.config.do_nudging {
            // Cap the nudge strength to a fraction of this generation's own relative score spread
            let effective_nudge_strength = self.config.nudge_strength
                .min(relative_score_spread(&scores) * self.config.nudge_relative_cap);

            let nudged_scores: Vec<f32> = self.population.iter()
                .zip(&scores)
                .map(|(chromosome, &score)| chromosome.nudged_score(score, effective_nudge_strength, self.config.energy_preference))
                .collect();

            sort_population_by_score(&mut self.population, nudged_scores);
        } else {
            sort_population_by_score(&mut self.population, scores);
        }

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(parent2, self.config.immigration_similarity_threshold);
            if rand::random_bool(self.effective_mutation_rate as f64) {
                child.mutate(self.effective_mutation_rate, self.config.immigration_similarity_threshold);
            }

            self.population[i] = child;
        }

        // Decay rates for next generation
        self.effective_survival_rate = (self.effective_survival_rate * (1.0 - survival_rate_decay)).max(0.0);
        self.effective_mutation_rate = (self.effective_mutation_rate * (1.0 - mutation_rate_decay)).max(mutation_rate_min);
        log::debug!("Generation {}: survival rate={:.3}, mutation rate={:.3}",
            self.generation, self.effective_survival_rate, self.effective_mutation_rate);

        // Fill remaining chromosomes by immigration
        let immigration_count = population_size.saturating_sub(immigration_start);
        for (offset, i) in (immigration_start..population_size).enumerate() {
            self.population[i] = Chromosome::immigrate(offset, immigration_count, &self.bounds);
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

/// Detect whether program behaviour shifted between generations.
///
/// We compare only chromosomes with a valid `prev_score`, so the score change is
/// meaningful. Heavily mutated chromosomes are ignored to avoid confusing workload
/// drift with unrelated configuration changes.
/// For each comparable chromosome, we compute the relative score change and take
/// the median. We also compute the median absolute deviation (MAD) of those changes.
/// Immigration triggers only when the median change is large enough and clearly
/// exceeds the spread of the paired deltas.
/// The goal is to answer a local question: did the workload change since the last
/// time these comparable configurations were observed? That keeps the logic simple
/// and avoids state that must be carried across many generations.
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

    log::debug!("Shift detection: {} comparable chromosomes", deltas.len());
    if deltas.len() < min_matched_scores {
        return false;
    }

    let median_delta = median(&mut deltas);
    log::trace!("Shift detection: delta={:.3}, threshold={:.3}", median_delta, change_threshold);
    if median_delta < change_threshold {
        return false;
    }

    let mut deviations: Vec<_> = deltas.into_iter().map(|delta| (delta - median_delta).abs()).collect();
    let mad = median(&mut deviations);
    let ratio = median_delta / (mad + f32::EPSILON);
    if ratio >= robustness_threshold {
        log::info!("Shift detected: change={:.2}%, robustness={:.2}", median_delta * 100.0, ratio);
        true
    } else {
        false
    }
}

/// Median absolute deviation of the scores relative to their median, i.e. a measure of
/// how spread out this generation's scores are, independent of their absolute magnitude.
fn relative_score_spread(xs: &[f32]) -> f32 {
    let median_score = median(&mut xs.to_vec());
    if median_score.abs() < f32::EPSILON {
        return 0.0;
    }

    let mut relative_devs: Vec<f32> = xs.iter()
        .map(|&s| (s - median_score).abs() / median_score)
        .collect();
    median(&mut relative_devs)
}
