use std::time::Duration;

use energy_bench::EnergyBench;
use rand::Rng;

use mtdynamic::Mtd;

fn data_fn() -> (Mtd, Vec<f32>) {
    const CYCLES: usize = 1_000_000;

    let mtd = Mtd::energy_controller(16);

    let mut rng = rand::thread_rng();
    let energy = (0..CYCLES).map(|_| rng.gen_range(0.001..10.0)).collect();

    (mtd, energy)
}

fn mtd_update((mut mtd, measurements): (Mtd, Vec<f32>)) {
    for energy in measurements {
        mtd.update(energy);
    }
}

fn main() {
    EnergyBench::new("overhead")
        .with_number_of_measurements(1)
        .with_min_measurement_duration(Duration::new(0, 0))
        .benchmark("overhead", &mtd_update, &data_fn);
}
