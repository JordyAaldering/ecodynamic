#[derive(Copy, Clone, Debug)]
#[derive(clap::ValueEnum)]
pub enum FilterFunction {
    Min,
    Average,
    Median,
    FrequencyDist,
}

impl FilterFunction {
    pub fn select(self, scores: Vec<f32>) -> f32 {
        use FilterFunction::*;
        match self {
            Min => min(scores),
            Average => average(scores),
            Median => median(scores),
            FrequencyDist => frequency_dist(scores, 5),
        }
    }
}

fn min(xs: Vec<f32>) -> f32 {
    xs.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

fn average(xs: Vec<f32>) -> f32 {
    let n = xs.len() as f32;
    xs.into_iter().sum::<f32>() / n
}

fn median(mut xs: Vec<f32>) -> f32 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs[xs.len() / 2]
}

fn frequency_dist(mut xs: Vec<f32>, num_ranges: usize) -> f32 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = xs[0];
    let max = xs[xs.len() - 1];
    let dist_size = (max - min) / num_ranges as f32;
    let mut dist_max = (1..=num_ranges).map(|i| min + dist_size * i as f32).collect::<Vec<f32>>();
    dist_max[num_ranges - 1] = max;

    let mut dist = vec![Vec::new(); num_ranges];
    let mut dist_index = 0;
    for x in xs {
        while x > dist_max[dist_index] {
            dist_index += 1;
        }

        dist[dist_index].push(x);
    }

    let biggest_dist = dist.into_iter().max_by_key(Vec::len).unwrap();
    biggest_dist[0]
}
