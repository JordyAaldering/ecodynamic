use std::hint::black_box;

use energy_bench::EnergyBenchBuilder;

use mtdynamic::Mtd;

const CYCLES: usize = 1_000_000;

fn make_mtd() -> Mtd {
    Mtd::energy_controller(16, 1)
}

fn mtd_update(mut mtd: Mtd) {
    for _ in 0..CYCLES {
        let _ = black_box(mtd.install(|| black_box(0)));
    }
}

fn main() {
    EnergyBenchBuilder::new("overhead")
        .build()
        .benchmark("overhead", &make_mtd, &mtd_update);
}
