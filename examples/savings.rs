#[path = "util/util.rs"]
mod util;
use rapl_energy::Rapl;
use util::*;

use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;

fn iter(mtd: &mut Mtd, iter: usize, size: usize, pin_threads: bool) -> (f32, f32) {
    let x = black_box(Matrix::random(size, size));
    let y = black_box(Matrix::random(size, size));

    let rapl = Rapl::now().unwrap();
    let start = Instant::now();

    for _ in 0..iter {
        let num_threads = mtd.num_threads() as usize;
        let pool = threadpool(num_threads, pin_threads);
        let _ = black_box(mtd.install(|| pool.install(|| x.mul(&y))));
    }

    let elapsed = start.elapsed();
    let energy = rapl.elapsed();
    (elapsed.as_secs_f32(), energy.into_values().sum())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let (max_threads, do_dynamic) = if let Some(max_threads) = args.get(1) {
        (max_threads.parse().unwrap(), false)
    } else {
        (16, true)
    };

    let mut mtd = if do_dynamic {
        Mtd::energy_controller(max_threads, 10)
    } else {
        Mtd::fixed_controller(max_threads)
    };

    // 1500x1500 takes about 10 times as long as 750x750
    let (r1, e1) = iter(&mut mtd, 1200, 750, true);
    let (r2, e2) = iter(&mut mtd, 100, 1500, true);
    let (r3, e3) = iter(&mut mtd, 100, 1500, false);
    let (r4, e4) = iter(&mut mtd, 1200, 750, false);

    println!("Runtime: {} + {} + {} + {}", r1, r2, r3, r4);
    println!("Energy: {} + {} + {} + {}", e1, e2, e3, e4);
    println!("{},{}", (r1 + r2 + r3 + r4), (e1 + e2 + e3 + e4));
}
