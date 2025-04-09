use crate::{Controller, Demand, Sample};

pub struct GeneticController {
    pub population: Vec<Chromosome>,
    samples: Option<Vec<Sample>>,
    /// Index of the chromosome for which we will return the thread-count
    population_idx: usize,
    // Configuration
    max_threads: i32,
    population_size: usize,
    survival_rate: f32,
    mutation_rate: f32,
}

impl GeneticController {
    pub fn new(max_threads: i32, population_size: usize, survival_rate: f32, mutation_rate: f32) -> Self {
        Self {
            population: (0..population_size).map(|_| Chromosome::rand(max_threads)).collect(),
            samples: None,
            population_idx: 0,
            max_threads,
            population_size,
            survival_rate,
            mutation_rate,
        }
    }

    fn evolve(&mut self) {
        self.population.iter_mut()
            .zip(self.samples.take().unwrap().into_iter())
            .for_each(|(chromosome, sample)| {
                chromosome.score = sample.energy;
            });

        self.population.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        // Keep the N% best performing chromosomes
        let n = (self.population_size as f32 * self.survival_rate).floor() as usize;
        // Replace the remaining chromosomes by children of the best performing chromosomes
        for i in n..self.population_size {
            let parent1 = &self.population[rand::random_range(0..n)];
            let parent2 = &self.population[rand::random_range(0..n)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_range(0.0..1.0) < self.mutation_rate {
                child.mutate(self.max_threads);
            }

            self.population[i] = child;
        }
    }
}

impl Controller for GeneticController {
    fn sample_received(&mut self, sample: Sample) {
        self.samples.get_or_insert_default().push(sample);

        if self.samples.as_ref().unwrap().len() >= self.population_size {
            self.evolve();
        }
    }

    fn next_demand(&mut self) -> Demand {
        // At this points the population is already sorted, the first element is the best-performing one
        self.population_idx = (self.population_idx + 1) % self.population_size;
        let num_threads = self.population[self.population_idx].num_threads;
        Demand { num_threads }
    }
}

#[derive(Clone)]
pub struct Chromosome {
    pub num_threads: i32,
    pub score: f32,
}

impl Chromosome {
    fn rand(max_threads: i32) -> Self {
        Self {
            num_threads: rand::random_range(1..=max_threads),
            score: 0.0,
        }
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            num_threads: (self.num_threads + other.num_threads) / 2,
            score: 0.0,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, max_threads: i32) {
        self.num_threads += rand::random_range(0..=1) * 2 - 1;
        self.num_threads = self.num_threads.max(1).min(max_threads)
    }
}
