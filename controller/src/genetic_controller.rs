use crate::{Builder, Controller, Demand, Sample};

pub struct GeneticBuilder {
    pub population_size: usize,
    pub survival_rate: f32,
    pub mutation_rate: f32,
}

pub struct GeneticController {
    pub population: Vec<Chromosome>,
    max_threads: i32,
    // Configuration
    population_size: usize,
    survival_rate: f32,
    mutation_rate: f32,
}

impl Builder<GeneticController> for GeneticBuilder {
    fn build(&self, max_threads: i32) -> GeneticController {
        let population = (0..self.population_size).map(|_| Chromosome::rand(max_threads)).collect();
        GeneticController {
            population,
            max_threads,
            population_size: self.population_size,
            survival_rate: self.survival_rate,
            mutation_rate: self.mutation_rate,
        }
    }
}

impl Controller for GeneticController {
    fn adjust_threads(&mut self, samples: Vec<Sample>) {
        let samples = samples.into_iter().map(|s| s.runtime);
        let mut fitness: Vec<(f32, &Chromosome)> = samples.zip(self.population.iter()).collect();
        fitness.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        // Keep the N% best performing chromosomes
        let n = (fitness.len() as f32 * self.survival_rate).floor() as usize;
        self.population = fitness.into_iter().take(n).map(|(_, c)| c.clone()).collect();

        // Fill the population through crossovers and mutations
        let mut children: Vec<Chromosome> = (0..(self.population_size - n)).map(|_| {
            let parent1 = &self.population[rand::random_range(0..n)];
            let parent2 = &self.population[rand::random_range(0..n)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_bool(self.mutation_rate as f64) {
                child.mutate(self.max_threads);
            }
            child
        }).collect();
        self.population.append(&mut children);
    }

    fn num_threads(&self) -> Demand {
        // At this points the population is already sorted, the first element is the best-performing one
        let num_threads = self.population[0].num_threads;
        Demand { num_threads }
    }
}

#[derive(Clone)]
pub struct Chromosome {
    pub num_threads: i32,
}

impl Chromosome {
    fn rand(max_threads: i32) -> Self {
        Self {
            num_threads: rand::random_range(1..=max_threads),
        }
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            num_threads: (self.num_threads + other.num_threads) / 2,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, max_threads: i32) {
        self.num_threads += rand::random_range(0..=1) * 2 - 1;
        self.num_threads = self.num_threads.max(1).min(max_threads)
    }
}
