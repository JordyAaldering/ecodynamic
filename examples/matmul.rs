#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 && args.len() != 5 {
        eprintln!("Usage: {} <size> <iter> <max-threads> <pin-threads>", args[0]);
        return;
    }

    let size: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let max_threads: usize = args[3].parse().unwrap();
    let pin_threads: bool = args.get(4).map_or(true, |x| x.parse().unwrap());

    let mut runtime: Vec<f32> = Vec::with_capacity(iter);
    let mut usertime: Vec<f32> = Vec::with_capacity(iter);
    let mut energy: Vec<f32> = Vec::with_capacity(iter);

    let mut rapl = Rapl::now().unwrap();

    let pool = mtdynamic::threadpool(max_threads, pin_threads);

    for _ in 0..iter {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        let _ = rapl.elapsed_mut();
        let user = ProcessTime::now();
        let real = Instant::now();

        pool.install(|| {
            let _ = black_box(x.mul(&y));
        });

        let real = real.elapsed();
        let user = user.elapsed();
        let rapl = rapl.elapsed_mut();

        runtime.push(real.as_secs_f32());
        usertime.push(user.as_secs_f32());
        energy.push(rapl.values().sum());
    }

    let n = iter as f32;
    println!("{:.8},{:.8},{:.8},{:.8},{:.8},{:.8}",
        runtime.iter().sum::<f32>() / n, stddev(&runtime),
        usertime.iter().sum::<f32>() / n, stddev(&usertime),
        energy.iter().sum::<f32>() / n, stddev(&energy),
    );
}
