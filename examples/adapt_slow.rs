#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::Mtd;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let (max_threads, do_dynamic) = if let Some(max_threads) = args.get(1) {
        (max_threads.parse().unwrap(), false)
    } else {
        (16, true)
    };

    const CYCLES: [(usize, bool); 14] = [
        // Without pinning
        (1200, false),
        (1150, false),
        (1100, false),
        (1050, false),
        (1000, false),
        ( 950, false),
        ( 900, false),
        // With pinning
        ( 900, true),
        ( 950, true),
        (1000, true),
        (1050, true),
        (1100, true),
        (1150, true),
        (1200, true),
    ];

    let mut mtd = if do_dynamic {
        Mtd::energy_controller(max_threads, 10)
    } else {
        Mtd::fixed_controller(max_threads)
    };

    let mut rapl = Rapl::now().unwrap();

    println!("size,pin,threads,runtime,usertime,energy");

    for (size, pin_threads) in CYCLES {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        for _ in 0..200 {
            rapl.reset();
            let user = ProcessTime::now();
            let real = Instant::now();

            let num_threads = mtd.num_threads() as usize;
            let pool = threadpool(num_threads, pin_threads);
            let _ = black_box(mtd.install(|| pool.install(|| x.mul(&y))));

            let real = real.elapsed();
            let user = user.elapsed();
            let rapl = rapl.elapsed();

            let real = real.as_secs_f32();
            let user = user.as_secs_f32();
            let energy: f32 = rapl.values().sum();

            println!("{},{},{},{},{},{}", size, pin_threads, mtd.num_threads, real, user, energy);
        }
    }
}
