pub fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}

pub fn median(xs: &mut Vec<f32>) -> f32 {
    debug_assert!(!xs.is_empty());
    xs.sort_unstable_by(f32::total_cmp);

    let n = xs.len();
    if n % 2 == 0 {
        (xs[n / 2 - 1] + xs[n / 2]) * 0.5
    } else {
        xs[n / 2]
    }
}

pub fn frequency_dist(mut xs: Vec<f32>, num_ranges: usize) -> f32 {
    xs.sort_unstable_by(f32::total_cmp);

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
