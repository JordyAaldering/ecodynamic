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
    average: bool,
}

impl FrequencyDist {
    pub fn new(num_ranges: usize, average: bool) -> Self {
        FrequencyDist { num_ranges, average }
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
    fn find_best(&self, mut samples: Vec<f64>) -> f64 {
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let dist_max = self.get_distribution_maximums(&samples);
        let mut dist = vec![Vec::new(); self.num_ranges];
        let mut dist_index = 0;
        for x in samples {
            while x > dist_max[dist_index] {
                dist_index += 1;
            }

            dist[dist_index].push(x);
        }

        let biggest_dist = dist.into_iter().max_by_key(Vec::len).unwrap();
        let len = biggest_dist.len() as f64;
        if self.average {
            biggest_dist.into_iter().sum::<f64>() / len
        } else {
            biggest_dist[0]
        }
    }
}
