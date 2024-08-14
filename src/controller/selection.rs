pub trait SelectionAlgorithm {
    fn find_best(&self, samples: Vec<u64>) -> u64;
}

pub struct Median {}

impl SelectionAlgorithm for Median {
    fn find_best(&self, mut samples: Vec<u64>) -> u64 {
        let idx = samples.len() / 2;
        samples.sort();
        samples[idx]
    }
}

pub struct Average {}

impl SelectionAlgorithm for Average {
    fn find_best(&self, samples: Vec<u64>) -> u64 {
        let len = samples.len() as u64;
        let total = samples.into_iter().sum::<u64>();
        total / len
    }
}

pub struct FrequencyDist {
    num_ranges: usize,
}

impl FrequencyDist {
    pub fn new(num_ranges: usize) -> Self {
        FrequencyDist { num_ranges }
    }

    fn get_distribution_maximums(&self, samples: &Vec<u64>) -> Vec<u64> {
        let min = samples[0];
        let max = samples[samples.len() - 1];
        let dist_size = (max - min) / self.num_ranges as u64;
        let mut res = (1..=self.num_ranges as u64)
            .map(|i| min + dist_size * i)
            .collect::<Vec<u64>>();
        res[self.num_ranges - 1] = max;
        res
    }
}

impl SelectionAlgorithm for FrequencyDist {
    fn find_best(&self, mut samples: Vec<u64>) -> u64 {
        samples.sort();

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
        biggest_dist[0]
    }
}
