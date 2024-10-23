#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::Mtd;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage: {} <size> <iter> <max-threads> <pin-threads> <controller-type>", args[0]);
        return;
    }

    let size: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let max_threads: usize = args[3].parse().unwrap();
    let pin_threads: bool = args[4].parse().unwrap();
    let controller_type = args[5].as_str();

    let mut reals: Vec<f32> = Vec::with_capacity(iter);
    let mut users: Vec<f32> = Vec::with_capacity(iter);
    let mut rapls: Vec<f32> = Vec::with_capacity(iter);

    let mut mtd = match controller_type {
        "energy" => Mtd::energy_controller(max_threads, 10),
        "runtime" => Mtd::runtime_controller(max_threads),
        "fixed" => Mtd::fixed_controller(max_threads),
        _ => unreachable!("Unknown controller type: {}", controller_type),
    };

    let mut rapl = Rapl::now().unwrap();

    for _ in 0..iter {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        rapl.reset();
        let user = ProcessTime::now();
        let real = Instant::now();

        let pool = threadpool(mtd.num_threads() as usize, pin_threads);
        let _ = black_box(mtd.install(|| pool.install(|| black_box(x.mul(&y)))));

        let real = real.elapsed();
        let user = user.elapsed();
        let rapl = rapl.elapsed();
        let real = real.as_secs_f32();
        let user = user.as_secs_f32();
        let rapl = rapl.values().sum();
        reals.push(real);
        users.push(user);
        rapls.push(rapl);

        //println!("{},{},{},{},{},{},{}", controller_type, size, pin_threads, mtd.num_threads, real, user, rapl);
    }

    let n = iter as f32;
    println!("{:.8},{:.8},{:.8},{:.8},{:.8},{:.8}",
        reals.iter().sum::<f32>() / n, stddev(&reals),
        users.iter().sum::<f32>() / n, stddev(&users),
        rapls.iter().sum::<f32>() / n, stddev(&rapls),
    );
}
