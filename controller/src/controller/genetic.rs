use clap::Parser;

use crate::{GlobalDemand, LocalDemand, Sample, ScoreFunction};

use super::Controller;

pub struct GeneticController {
    population: Vec<Chromosome>,
    sort_order: Order,
    sample_index: usize,
    config: GeneticControllerConfig,
}

enum Order {
    Increasing,
    Decreasing,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct GeneticControllerConfig {
    #[arg(long, default_value_t = ScoreFunction::Pareto)]
    pub score: ScoreFunction,

    /// Minimum allowed percentage of the number of threads (0,1]
    #[arg(long, default_value_t = 0.1)]
    pub threads_rate_min: f32,
    /// Maximum allowed percentage of the number of threads (0,1]
    #[arg(long, default_value_t = 1.0)]
    pub threads_rate_max: f32,

    /// Minimum allowed percentage of the powercap (0,1]
    #[arg(long, default_value_t = 0.1)]
    pub power_rate_min: f32,
    /// Maximum allowed percentage of the powercap (0,1]
    #[arg(long, default_value_t = 1.0)]
    pub power_rate_max: f32,

    /// Genetic algorithm survival rate.
    #[arg(long, default_value_t = 0.50)]
    pub survival_rate: f32,
    /// Mutation rate.
    #[arg(long, default_value_t = 0.25)]
    pub mutation_rate: f32,
    /// Mutation strength (0,1]
    #[arg(long, default_value_t = 0.1)]
    pub mutation_strength: f32,
    /// Immigration rate.
    /// Immigration can result in very poor chromosomes and might thus be very costly. We want to
    /// avoid immigration to occur in every evolution step. Setting the value to less than
    /// 1 / population_size ensures this.
    /// We have two choices for resolving this:
    ///  1. Keep track of how many evolutions we have done and base immigration count on that total.
    ///  2. Or, during every evolution apply immigration based on random change.
    #[arg(long, default_value_t = 0.0)]
    pub immigration_rate: f32,
}

impl GeneticController {
    pub fn new(population_size: usize, config: GeneticControllerConfig) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        // I.e. value = lower + i * (upper - lower) / length
        let population = (0..population_size).map(|i| {
                let threads_pct = config.threads_rate_min + (i as f32 * (config.threads_rate_max - config.threads_rate_min) / (population_size - 1) as f32);
                let power_pct = config.power_rate_min + (i as f32 * (config.power_rate_max - config.power_rate_min) / (population_size - 1) as f32);
                Chromosome::new(threads_pct, power_pct)
            }).collect();

        Self {
            population,
            sort_order: Order::Increasing,
            sample_index: 0,
            config,
        }
    }
}

impl Controller for GeneticController {
    fn evolve(&mut self, samples: Vec<Sample>) {
        // Reset sample index for next
        self.sample_index = 0;

        let scores = self.config.score.score(samples);
        sort_population_by_score(&mut self.population, scores);

        let population_size = self.population.len();
        let survival_count = (population_size as f32 * self.config.survival_rate).round() as usize;
        let immigration_count = (population_size as f32 * self.config.immigration_rate).round() as usize;
        let immigration_start = population_size - immigration_count;

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_bool(self.config.mutation_rate as f64) {
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
            Order::Increasing => {
                self.population.sort_by(|a, b| a.partial_cmp(&b).unwrap());
                self.sort_order = Order::Decreasing;
            },
            Order::Decreasing => {
                self.population.sort_by(|a, b| b.partial_cmp(&a).unwrap());
                self.sort_order = Order::Increasing;
            }
        }
    }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        // Use the number of samples to determine the current index into the population.
        // The population is reset every `population_size` iterations.
        // In between, we want every chromosome to be applied once.
        let chromosome = &self.population[self.sample_index];
        self.sample_index += 1;

        let global = GlobalDemand { power_limit_pct: chromosome.power_pct };
        let local = LocalDemand { threads_pct: chromosome.threads_pct };
        (global, local)
    }
}

fn sort_population_by_score<T>(population: &mut Vec<T>, scores: Vec<f32>) {
    let mut permutation = permutation::sort_by(&scores, |a, b| a.partial_cmp(b).unwrap());
    permutation.apply_slice_in_place(population);
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
        let num_threads = rand::random_range(config.threads_rate_min..=config.threads_rate_max);
        let power_limit_pct = rand::random_range(config.power_rate_min..=config.power_rate_max);
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
        self.threads_pct = self.threads_pct.max(config.threads_rate_min).min(config.threads_rate_max);

        self.power_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.power_pct = self.power_pct.max(config.power_rate_min).min(config.power_rate_max);
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.power_pct.partial_cmp(&other.power_pct)
    }
}
