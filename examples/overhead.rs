use std::hint::black_box;

use energy_bench::EnergyBenchBuilder;

use mtdynamic::Mtd;

const CYCLES: usize = 1_000_000;

fn make_mtd(samples_per_update: usize) -> Mtd {
    Mtd::energy_controller(16, samples_per_update)
}

fn mtd_update(mut mtd: Mtd) {
    for _ in 0..CYCLES {
        let _ = black_box(mtd.install(|| black_box(0)));
    }
}

fn main() {
    let mut bench = EnergyBenchBuilder::new("overhead")
        .build();

    bench.benchmark(1, &|| make_mtd(1), &mtd_update);
    bench.benchmark(10, &|| make_mtd(10), &mtd_update);
    bench.benchmark(20, &|| make_mtd(20), &mtd_update);
}
