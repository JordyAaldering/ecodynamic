use std::time::Duration;

use energy_bench::EnergyBench;
use rand::Rng;

use mtdynamic::{MTDynamic, MtdBuilder};

fn data_fn() -> (MTDynamic, Vec<(f64, f64, f64)>) {
    const CYCLES: usize = 1_000_000;

    let mtd = MtdBuilder::new(16).letterbox_size(1).build();

    let mut rng = rand::thread_rng();
    let measurements = (0..CYCLES).map(|_| {
        let runtime = rng.gen_range(0.001..10.0);
        let usertime = rng.gen_range(0.001..10.0);
        let energy = rng.gen_range(0.001..10.0);
        (runtime, usertime, energy)
    }).collect();

    (mtd, measurements)
}

fn mtd_update((mut mtd, measurements): (MTDynamic, Vec<(f64, f64, f64)>)) {
    for (runtime, usertime, energy) in measurements {
        mtd.update("overhead", runtime, usertime, energy);
    }
}

fn main() {
    EnergyBench::new("overhead")
        .with_number_of_measurements(1)
        .with_min_measurement_duration(Duration::new(0, 0))
        .benchmark("overhead", &mtd_update, &data_fn);
}
