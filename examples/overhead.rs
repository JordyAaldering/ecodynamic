use std::{hint::black_box, time::Duration};

use energy_bench::EnergyBench;

use mtdynamic::Mtd;

const CYCLES: usize = 1_000_000;

fn make_mtd() -> Mtd {
    Mtd::energy_controller(16)
}

fn mtd_update(mut mtd: Mtd) {
    for _ in 0..CYCLES {
        let mut rapl = rapl_energy::Rapl::now().unwrap();
        let _ = black_box(|| black_box(0));
        let energy = rapl.elapsed_mut().values().sum();
        mtd.update(energy);
    }
}

fn main() {
    EnergyBench::new("overhead")
        .with_number_of_measurements(1)
        .with_min_measurement_duration(Duration::new(0, 0))
        .benchmark("overhead", &mtd_update, &make_mtd);
}
