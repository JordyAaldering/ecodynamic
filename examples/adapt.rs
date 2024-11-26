#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;
use rapl_energy::{EnergyProbe, Rapl};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let max_threads: usize = args[1].parse().unwrap();

    let mut mtd = Mtd::energy_controller(max_threads, 10);
    let mut rapl = Rapl::now(false).unwrap();

    println!("threads,size,pin,runtime,energy");

    for size in [500, 1000, 1500] {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        for _ in 0..250 {
            let num_threads = mtd.num_threads() as usize;
            let pool = threadpool(num_threads);

            rapl.reset();
            let instant = Instant::now();

            black_box(mtd.install(|| pool.install(|| x.mul(&y))));

            let runtime = instant.elapsed();
            let energy = rapl.elapsed();

            let runtime = runtime.as_secs_f32();
            let energy = energy.values().sum::<f32>();
            println!("{},{},{},{}", mtd.num_threads, size, runtime, energy);
        }
    }
}
