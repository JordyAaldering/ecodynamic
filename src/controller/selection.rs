pub trait SelectionAlgorithm {
    fn find_best(&self, samples: Vec<f64>) -> f64;
}

pub struct Median {}

impl SelectionAlgorithm for Median {
    fn find_best(&self, mut samples: Vec<f64>) -> f64 {
        let idx = samples.len() / 2;
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        samples[idx]
    }
}

pub struct FrequencyDist {
    num_ranges: usize,
}

impl FrequencyDist {
    pub fn new(num_ranges: usize) -> Self {
        FrequencyDist { num_ranges }
    }

    pub fn distribute(&self, mut samples: Vec<f64>) -> Vec<Vec<f64>> {
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let dist_max = self.get_distribution_maximums(&samples);
        let mut distributions = vec![Vec::new(); self.num_ranges];
        let mut dist_index = 0;
        for x in samples {
            while x > dist_max[dist_index] {
                dist_index += 1;
            }

            distributions[dist_index].push(x);
        }

        distributions
    }

    fn get_distribution_maximums(&self, samples: &Vec<f64>) -> Vec<f64> {
        let min = samples[0];
        let max = samples[samples.len() - 1];
        let dist_size = (max - min) / self.num_ranges as f64;
        let mut res = (1..=self.num_ranges)
            .map(|i| min + dist_size * i as f64)
            .collect::<Vec<f64>>();
        res[self.num_ranges - 1] = max;
        res
    }
}

impl SelectionAlgorithm for FrequencyDist {
    fn find_best(&self, samples: Vec<f64>) -> f64 {
        let distributions = self.distribute(samples);
        let biggest_dist = distributions.into_iter().max_by_key(Vec::len).unwrap();
        biggest_dist[0]
    }
}
