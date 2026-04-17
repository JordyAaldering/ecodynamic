use clap::Parser;

use crate::{Capabilities, Controller, Demand, Sample, direction::Direction, scoring_functions::ScoreFunction};

pub struct GeneticController {
    samples: Vec<Sample>,
    population: Vec<Chromosome>,
    immigration_detector: EwmaDetector,
    immigration_cooldown: usize,
    sort_order: Direction,
    max_threads: u16,
    config: GeneticControllerConfig,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct GeneticControllerConfig {
    #[arg(short('s'), long, default_value_t = 30)]
    pub population_size: usize,

    /// Method for scoring the fitness of each chromosome.
    #[arg(long, default_value_t = ScoreFunction::Slider)]
    pub score: ScoreFunction,

    /// If the `Slider` scoring function is used, this value describes how important
    /// energy consumption is in the optimisation process. The importance of runtime
    /// is then 1 minus this value.
    /// Range: [0,1]
    #[arg(long, default_value_t = 0.75)]
    pub energy_preference: f32,

    /// Whether dynamic thread adjustment is enabled.
    #[arg(long)]
    pub do_thread_control: bool,

    /// Minimum allowed percentage of the powercap.
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.1)]
    pub power_min: f32,

    /// Maximum allowed percentage of the powercap.
    /// Range: (0,1].
    #[arg(long, default_value_t = 1.0)]
    pub power_max: f32,

    /// Genetic algorithm survival rate.
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.15)]
    pub survival_rate: f32,

    /// Mutation rate.
    /// Range: (0,1]
    #[arg(long, default_value_t = 0.30)]
    pub mutation_rate: f32,

    /// Mutation strength.
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.005)]
    pub mutation_strength: f32,

    /// Immigration can result in very poor chromosomes and might thus be very costly. We want to
    /// avoid immigration to occur in every evolution step. Setting the value to less than
    /// 1 / population_size ensures this.
    /// Range: (0,1]
    #[arg(long)]
    pub immigration_rate: Option<f32>,

    /// EWMA smoothing factor for immigration trigger detection.
    /// Range: (0,1].
    #[arg(long, default_value_t = 0.2)]
    pub immigration_ewma_alpha: f32,

    /// EWMA z-score threshold for immigration trigger detection.
    #[arg(long, default_value_t = 3.0)]
    pub immigration_ewma_z_threshold: f32,

    /// Number of consecutive EWMA threshold breaches needed to trigger immigration.
    #[arg(long, default_value_t = 1)]
    pub immigration_ewma_consecutive_breaches: usize,

    /// Number of generations to wait before allowing immigration to trigger again.
    #[arg(long, default_value_t = 3)]
    pub immigration_cooldown_generations: usize,
}

impl GeneticController {
    pub fn new(config: GeneticControllerConfig, caps: &Capabilities) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        // I.e. value = lower + i * (upper - lower) / length
        let population = (0..config.population_size)
            .map(|i| {
                let threads_pct = if config.do_thread_control {
                    0.1 + (i as f32 * (1.0 - 0.1) / (config.population_size - 1) as f32)
                } else {
                    1.0
                };
                let power_pct = config.power_min
                    + (i as f32 * (config.power_max - config.power_min)
                        / (config.population_size - 1) as f32);
                Chromosome::new(threads_pct, power_pct)
            })
            .rev()
            .collect();

        log::trace!("Init: {:?}", population);

        Self {
            samples: Vec::with_capacity(config.population_size),
            population,
            immigration_detector: EwmaDetector::new(
                config.immigration_ewma_alpha,
                config.immigration_ewma_z_threshold,
                config.immigration_ewma_consecutive_breaches,
            ),
            immigration_cooldown: 0,
            sort_order: Direction::Decreasing,
            max_threads: caps.max_threads.unwrap_or(1),
            config,
        }
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
        let GeneticControllerConfig {
            score: score_fn,
            energy_preference,
            survival_rate,
            mutation_rate,
            immigration_rate,
            immigration_cooldown_generations,
            ..
        } = self.config;

        let scores = score_fn.score(&self.samples, energy_preference);
        let change_signal = update_prev_scores_and_get_change_signal(&mut self.population, &scores);

        let population_size = self.population.len();

        // When survival rate is less than 1 / population_size, we use a random
        // chance based on the remainder to ensure survival can still occur.
        let survival_count = {
            let survival_count = population_size as f32 * survival_rate;
            let survival_remainder = survival_count.fract();
            let mut survival_count = survival_count.floor() as usize;
            if rand::random_bool(survival_remainder as f64) {
                survival_count += 1;
            }
            survival_count
        };

        let immigration_start = if let Some(immigration_rate) = immigration_rate {
            let do_immigration = if self.immigration_cooldown > 0 {
                self.immigration_cooldown -= 1;
                false
            } else {
                let change_detected = change_signal.is_some_and(|signal| self.immigration_detector.update(signal));
                if change_detected {
                    self.immigration_cooldown = immigration_cooldown_generations;
                    true
                } else {
                    false
                }
            };

            if do_immigration {
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
        } else {
            population_size
        };

        sort_population_by_score(&mut self.population, scores);

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(parent2);
            if rand::random_bool(mutation_rate as f64) {
                child.mutate(&self.config);
            }

            self.population[i] = child;
        }

        // Fill remaining chromosomes by immigration
        for i in immigration_start..population_size {
            self.population[i] = Chromosome::rand(&self.config);
        }

        // To minimise changes in the runtime we sort by the recommended power limit
        // and we oscilate between an increasing and decreasing order.
        match self.sort_order {
            Direction::Increasing => {
                self.population.sort_by(|a, b| a.partial_cmp(b).unwrap());
                self.sort_order = Direction::Decreasing;
            }
            Direction::Decreasing => {
                self.population.sort_by(|a, b| b.partial_cmp(a).unwrap());
                self.sort_order = Direction::Increasing;
            }
        }

        log::trace!("Evolve: {:?}", self.population);
    }
}

fn sort_population_by_score(population: &mut Vec<Chromosome>, scores: Vec<f32>) {
    let mut combined: Vec<_> = population.drain(..).zip(scores).collect();
    combined.sort_unstable_by(|(_, a), (_, b)| a.total_cmp(b));
    *population = combined.into_iter().map(|(c, _)| c).collect();
}

fn update_prev_scores_and_get_change_signal(population: &mut [Chromosome], scores: &[f32]) -> Option<f32> {
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

    median(&mut deltas)
}

fn median(values: &mut Vec<f32>) -> Option<f32> {
    if values.is_empty() {
        return None;
    }

    values.sort_unstable_by(|a, b| a.total_cmp(b));
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        Some((values[mid - 1] + values[mid]) * 0.5)
    } else {
        Some(values[mid])
    }
}

#[derive(Clone, Debug)]
struct EwmaDetector {
    alpha: f32,
    z_threshold: f32,
    min_consecutive_breaches: usize,
    consecutive_breaches: usize,
    mean: f32,
    variance: f32,
    initialized: bool,
}

impl EwmaDetector {
    fn new(alpha: f32, z_threshold: f32, min_consecutive_breaches: usize) -> Self {
        Self {
            alpha,
            z_threshold,
            min_consecutive_breaches,
            consecutive_breaches: 0,
            mean: 0.0,
            variance: 0.0,
            initialized: false,
        }
    }

    fn update(&mut self, x: f32) -> bool {
        if !self.initialized {
            self.mean = x;
            self.variance = 0.0;
            self.initialized = true;
            self.consecutive_breaches = 0;
            return false;
        }

        let prev_mean = self.mean;
        let prev_variance = self.variance;
        let std = prev_variance.max(f32::EPSILON).sqrt();
        let z = ((x - prev_mean) / std).abs();

        if z > self.z_threshold {
            self.consecutive_breaches += 1;
        } else {
            self.consecutive_breaches = 0;
        }

        self.mean = self.alpha * x + (1.0 - self.alpha) * prev_mean;
        let residual = x - prev_mean;
        self.variance = self.alpha * residual * residual + (1.0 - self.alpha) * prev_variance;

        self.consecutive_breaches >= self.min_consecutive_breaches
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

    fn crossover(&self, other: &Self) -> Self {
        Self {
            threads_pct: (self.threads_pct + other.threads_pct) * 0.5,
            power_pct: (self.power_pct + other.power_pct) * 0.5,
            prev_score: self.prev_score.and_then(|x| other.prev_score.map(|y| (x + y) * 0.5)),
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, config: &GeneticControllerConfig) {
        self.threads_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.threads_pct = self.threads_pct.max(0.1).min(1.0);

        self.power_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.power_pct = self.power_pct.max(config.power_min).min(config.power_max);
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.power_pct.partial_cmp(&other.power_pct)
    }
}
