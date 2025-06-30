use clap::Parser;

use crate::{GlobalDemand, LocalDemand, Sample, ScoreFunction};

use super::Controller;

pub struct GeneticController {
    population: Vec<Chromosome>,
    sample_index: usize,
    config: GeneticControllerConfig,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct GeneticControllerConfig {
    #[arg(long, default_value_t = ScoreFunction::Pareto)]
    pub score: ScoreFunction,

    /// Minimum allowed percentage of the number of threads (0,1]
    #[arg(long, default_value_t = 0.1)]
    pub threads_rate_min: f32,

    /// Minimum allowed percentage of the powercap (0,1]
    #[arg(long, default_value_t = 0.1)]
    pub power_rate_min: f32,

    /// Genetic algorithm survival rate.
    #[arg(long, default_value_t = 0.50)]
    pub survival_rate: f32,

    /// Mutation rate.
    #[arg(long, default_value_t = 0.25)]
    pub mutation_rate: f32,

    /// Mutation strength (0,1]
    #[arg(long, default_value_t = 0.25)]
    pub mutation_strength: f32,

    /// Immigration rate.
    #[arg(long, default_value_t = 0.0)]
    pub immigration_rate: f32,
}

impl GeneticController {
    pub fn new(population_size: usize, config: GeneticControllerConfig) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        // I.e. value = lower + i * (upper - lower) / length
        let population = (0..population_size).map(|i| {
                let threads_pct = config.threads_rate_min + (i as f32 * (1.0 - config.threads_rate_min) / (population_size - 1) as f32);
                let power_pct = config.power_rate_min + (i as f32 * (1.0 - config.power_rate_min) / (population_size - 1) as f32);
                Chromosome::new(threads_pct, power_pct)
            }).collect();

        Self {
            population,
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
        sort(scores, &mut self.population);

        let population_size = self.population.len();
        let survival_count = (population_size as f32 * self.config.survival_rate).round() as usize;
        let immigration_count = (population_size as f32 * self.config.immigration_rate).round() as usize;
        let immigration_start = population_size - immigration_count;

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_range(0.0..1.0) < self.config.mutation_rate {
                child.mutate(&self.config);
            }

            self.population[i] = child;
        }

        // Fill remaining chromosomes by immigration
        for i in immigration_start..population_size {
            self.population[i] = Chromosome::rand(&self.config);
        }

        // To minimise changes in the runtime we sort by the recommended power limit
        self.population.sort_by(|a, b| a.power_pct.partial_cmp(&b.power_pct).unwrap());
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

fn sort<T>(scores: Vec<f32>, population: &mut Vec<T>) {
    let mut permutation = permutation::sort_by(&scores, |a, b| a.partial_cmp(b).unwrap());
    permutation.apply_slice_in_place(population);
}

#[derive(Clone, Debug)]
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
        //let num_threads = rand::random_range(THREADS_PCT_MIN..=1.0);
        let num_threads = rand::random_range(config.threads_rate_min..=1.0);
        let power_limit_pct = rand::random_range(config.power_rate_min..=1.0);
        Self::new(num_threads, power_limit_pct)
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            threads_pct: (self.threads_pct + other.threads_pct) / 2.0,
            power_pct: (self.power_pct + other.power_pct) / 2.0,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, config: &GeneticControllerConfig) {
        self.threads_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.threads_pct = self.threads_pct.max(config.threads_rate_min).min(1.0);

        self.power_pct += rand::random_range(-config.mutation_strength..=config.mutation_strength);
        self.power_pct = self.power_pct.max(config.power_rate_min).min(1.0);
    }
}

#[cfg(test)]
mod tests {
    /// Population should be sorted from lowest to highest runtime/energy consumption.
    #[test]
    fn test_sort() {
        let vals = vec![0.2, 0.1, 0.4, 0.3];
        let mut idxs = vec![2, 1, 4, 3];

        super::sort(vals, &mut idxs);

        assert_eq!(idxs, vec![1, 2, 3, 4]);
    }
}
