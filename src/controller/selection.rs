pub(super) trait SelectionAlgorithm {
    fn find_best_time(&self, samples: &Vec<u64>) -> u64;
}

pub(super) struct Average {}

impl SelectionAlgorithm for Average {
    fn find_best_time(&self, samples: &Vec<u64>) -> u64 {
        samples.iter().sum::<u64>() / samples.len() as u64
    }
}

pub(super) struct FrequencyDist {
    pub(super) num_ranges: usize,
}

impl SelectionAlgorithm for FrequencyDist {
    fn find_best_time(&self, samples: &Vec<u64>) -> u64 {
        let min = *samples.iter().filter(|&&x| x > 0).min().unwrap();
        let max = *samples.iter().filter(|&&x| x > 0).max().unwrap();
        let dist_size = (max - min) / self.num_ranges as u64;

        let dist_max: Vec<u64> = (1..=self.num_ranges).map(|i| min + dist_size * i as u64).collect();

        let mut distributions = vec![Vec::<u64>::new(); self.num_ranges];
        for &x in samples {
            for (i, &dmax) in dist_max.iter().enumerate() {
                if x < dmax {
                    distributions[i].push(x);
                    break;
                }
            }
        }

        let biggest = distributions.iter().max_by_key(|x| x.len()).unwrap();
        *biggest.iter().min().unwrap()
    }
}
