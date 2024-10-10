#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::MtdBuilder;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <size> <iter> <threads> <pin_threads?>", args[0]);
        return;
    }

    let size: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let num_threads: usize = args[3].parse().unwrap();
    let pin_threads: bool = args[4].parse().unwrap();

    let mut runtime: Vec<f64> = Vec::with_capacity(iter);
    let mut usertime: Vec<f64> = Vec::with_capacity(iter);
    let mut energy: Vec<f64> = Vec::with_capacity(iter);

    let mut rapl = Rapl::now().unwrap();

    let mut mtd = MtdBuilder::new(num_threads as i32).build();
    let mut pool = threadpool(num_threads, pin_threads);

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

        let real = real.as_secs_f64();
        let user = user.as_secs_f64();
        let rapl = rapl.values().sum();

        runtime.push(real);
        usertime.push(user);
        energy.push(rapl);

        mtd.update("parallel", real, user, rapl);
        let t = mtd.num_threads("parallel") as usize;
        if pool.current_num_threads() != t {
            pool = threadpool(num_threads, pin_threads);
        }
    }

    println!("{:.8},{:.8},{:.8}",
        runtime.into_iter().sum::<f64>() / iter as f64,
        usertime.into_iter().sum::<f64>() / iter as f64,
        energy.into_iter().sum::<f64>() / iter as f64,
    );
}
