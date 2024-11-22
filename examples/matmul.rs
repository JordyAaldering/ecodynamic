#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;
use rapl_energy::{EnergyProbe, Rapl};

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

    let mut mtd = match controller_type {
        "energy" => Mtd::energy_controller(max_threads, 10),
        "runtime" => Mtd::runtime_controller(max_threads),
        "fixed" => Mtd::fixed_controller(max_threads),
        _ => unreachable!("Unknown controller type: {}", controller_type),
    };

    let mut runtimes: Vec<f32> = Vec::with_capacity(iter);
    let mut energies: Vec<f32> = Vec::with_capacity(iter);

    let mut rapl = Rapl::now(false).unwrap();

    for _ in 0..iter {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        rapl.reset();
        let instant = Instant::now();

        let num_threads = mtd.num_threads() as usize;
        let pool = threadpool(num_threads, pin_threads);
        let _ = black_box(mtd.install(|| pool.install(|| x.mul(&y))));

        let runtime = instant.elapsed();
        let energy = rapl.elapsed();
        let runtime = runtime.as_secs_f32();
        let energy = energy.values().sum();
        runtimes.push(runtime);
        energies.push(energy);
    }

    let n = iter as f32;
    println!("{:.8},{:.8},{:.8},{:.8}",
        runtimes.iter().sum::<f32>() / n, stddev(&runtimes),
        energies.iter().sum::<f32>() / n, stddev(&energies),
    );
}
