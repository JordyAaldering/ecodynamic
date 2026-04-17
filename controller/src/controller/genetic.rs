use clap::Parser;

use crate::{Capabilities, Controller, Demand, Sample, direction::Direction, scoring_functions::ScoreFunction};

pub struct GeneticController {
    samples: Vec<Sample>,
    population: Vec<Chromosome>,
    immigration_detector: PageHinkley,
    immigration_change_detected: bool,
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
    #[arg(long, default_value_t = 0.0)]
    pub immigration_rate: f32,

    /// Trigger immigration only when changes are detected.
    /// If not set, immigration is always enabled (subject to immigration rate).
    #[arg(long)]
    pub immigration_trigger: Option<f32>,

    /// Page-Hinkley tolerance parameter used for immigration trigger detection.
    #[arg(long, default_value_t = 0.005)]
    pub immigration_ph_delta: f32,

    /// Page-Hinkley threshold used for immigration trigger detection.
    #[arg(long, default_value_t = 0.05)]
    pub immigration_ph_lambda: f32,

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
            immigration_detector: PageHinkley::new(
                config.immigration_ph_delta,
                config.immigration_ph_lambda,
            ),
            immigration_change_detected: false,
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
        if self.config.immigration_trigger.is_some() {
            let signal = score_single_sample(
                self.config.score,
                &sample,
                self.config.energy_preference,
            );
            if self.immigration_detector.update(signal) {
                self.immigration_change_detected = true;
                self.immigration_detector.reset();
            }
        }

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
            immigration_trigger,
            immigration_cooldown_generations,
            ..
        } = self.config;

        let scores = score_fn.score(&self.samples, energy_preference);

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

        let do_immigration = if immigration_trigger.is_some() {
            if self.immigration_cooldown > 0 {
                self.immigration_cooldown -= 1;
                self.immigration_change_detected = false;
                false
            } else if self.immigration_change_detected {
                self.immigration_cooldown = immigration_cooldown_generations;
                self.immigration_change_detected = false;
                true
            } else {
                false
            }
        } else {
            // Immigration trigger is not set; immigration is always enabled
            // (By default, there will still be no immigration because the immigration rate is 0)
            true
        };

        let immigration_start = if do_immigration {
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

fn score_single_sample(score_fn: ScoreFunction, sample: &Sample, energy_preference: f32) -> f32 {
    match score_fn {
        ScoreFunction::Runtime => sample.runtime,
        ScoreFunction::Energy => sample.energy,
        ScoreFunction::EDP => sample.energy * sample.runtime,
        ScoreFunction::E2DP => sample.energy * sample.energy * sample.runtime,
        ScoreFunction::Slider => sample.energy.powf(energy_preference)
            * sample.runtime.powf(1.0 - energy_preference),
        // Pareto ranking needs a full population. For online drift detection we use
        // the scalar slider score as a low-overhead proxy.
        ScoreFunction::Pareto => sample.energy.powf(energy_preference)
            * sample.runtime.powf(1.0 - energy_preference),
    }
}

#[derive(Clone, Debug)]
struct PageHinkley {
    delta: f32,
    lambda: f32,
    n: usize,
    mean: f32,
    cumulative_sum_pos: f32,
    min_cumulative_sum_pos: f32,
    cumulative_sum_neg: f32,
    max_cumulative_sum_neg: f32,
}

impl PageHinkley {
    fn new(delta: f32, lambda: f32) -> Self {
        Self {
            delta,
            lambda,
            n: 0,
            mean: 0.0,
            cumulative_sum_pos: 0.0,
            min_cumulative_sum_pos: 0.0,
            cumulative_sum_neg: 0.0,
            max_cumulative_sum_neg: 0.0,
        }
    }

    fn update(&mut self, x: f32) -> bool {
        self.n += 1;
        let inv_n = 1.0 / self.n as f32;
        self.mean += (x - self.mean) * inv_n;

        // Two-sided Page-Hinkley to detect both upward and downward shifts.
        self.cumulative_sum_pos += x - self.mean - self.delta;
        self.min_cumulative_sum_pos = self.min_cumulative_sum_pos.min(self.cumulative_sum_pos);

        self.cumulative_sum_neg += x - self.mean + self.delta;
        self.max_cumulative_sum_neg = self.max_cumulative_sum_neg.max(self.cumulative_sum_neg);

        let positive_shift = (self.cumulative_sum_pos - self.min_cumulative_sum_pos) > self.lambda;
        let negative_shift = (self.max_cumulative_sum_neg - self.cumulative_sum_neg) > self.lambda;
        positive_shift || negative_shift
    }

    fn reset(&mut self) {
        self.n = 0;
        self.mean = 0.0;
        self.cumulative_sum_pos = 0.0;
        self.min_cumulative_sum_pos = 0.0;
        self.cumulative_sum_neg = 0.0;
        self.max_cumulative_sum_neg = 0.0;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chromosome {
    threads_pct: f32,
    power_pct: f32,
}

impl Chromosome {
    fn new(threads_pct: f32, power_pct: f32) -> Self {
        Self { threads_pct, power_pct }
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
