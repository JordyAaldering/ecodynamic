#[path = "util/util.rs"]
mod util;
use rapl_energy::Rapl;
use util::*;

use std::{hint::black_box, time::{Duration, Instant}};

use mtdynamic::Mtd;

const ITER_TIME: Duration = Duration::from_secs(5 * 60);

fn iter(mtd: &mut Mtd, size: usize, pin_threads: bool) -> f32 {
    let x = black_box(Matrix::random(size, size));
    let y = black_box(Matrix::random(size, size));

    let rapl = Rapl::now().unwrap();

    let start = Instant::now();
    while start.elapsed() < ITER_TIME {
        let num_threads = mtd.num_threads() as usize;
        let pool = threadpool(num_threads, pin_threads);
        let _ = black_box(mtd.install(|| pool.install(|| x.mul(&y))));
    }

    rapl.elapsed().into_values().sum()
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

    let mut energy = 0.0;
    energy += iter(&mut mtd, 750, true);
    energy += iter(&mut mtd, 1250, true);
    energy += iter(&mut mtd, 1250, false);

    let runtime = ITER_TIME.as_secs_f32() * 3.0;
    println!("{},{}", runtime, energy);
}
