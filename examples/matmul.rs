#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use rapl_energy::{EnergyProbe, Rapl};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let max_threads: usize = args[2].parse().unwrap();

    let mut runtimes: Vec<f32> = Vec::with_capacity(250);
    let mut energies: Vec<f32> = Vec::with_capacity(250);

    let mut rapl = Rapl::now(false).unwrap();
    let pool = threadpool(max_threads);

    for _ in 0..250 {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        rapl.reset();
        let instant = Instant::now();

        black_box(pool.install(|| x.mul(&y)));

        let runtime = instant.elapsed();
        let energy = rapl.elapsed();
        runtimes.push(runtime.as_secs_f32());
        energies.push(energy.values().sum());
    }

    println!("{:.8},{:.8},{:.8},{:.8}",
        runtimes.iter().sum::<f32>() / 250.0, stddev(&runtimes),
        energies.iter().sum::<f32>() / 250.0, stddev(&energies),
    );
}
