#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::Mtd;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let print_intermediate: bool = args[1].parse().unwrap();
    let mut max_threads = 16;
    let mut dynamic = true;
    if args.len() > 2 {
        max_threads = args[2].parse().unwrap();
        dynamic = true;
    }

    const CYCLES: [(usize, bool); 16] = [
        // Without pinning
        (850, false),
        (900, false),
        (950, false),
        (1000, false),
        (1050, false),
        (1100, false),
        (1150, false),
        (1200, false),
        // With pinning
        (1200, true),
        (1150, true),
        (1100, true),
        (1050, true),
        (1000, true),
        (950, true),
        (900, true),
        (850, true),
    ];

    let mut mtd = Mtd::energy_controller(max_threads);
    let mut rapl = Rapl::now().unwrap();

    if print_intermediate {
        println!("size,pin,threads,runtime,usertime,energy");
    }

    let mut real_total = 0.0;
    let mut user_total = 0.0;
    let mut rapl_total = 0.0;

    for (size, pin_threads) in CYCLES {
        for _ in 0..200 {
            let x = black_box(Matrix::random(size, size));
            let y = black_box(Matrix::random(size, size));

            let _ = rapl.elapsed_mut();
            let user = ProcessTime::now();
            let real = Instant::now();

            let _res = if dynamic {
                mtd.install(pin_threads, || black_box(x.mul(&y)))
            } else {
                black_box(x.mul(&y))
            };

            let real = real.elapsed();
            let user = user.elapsed();
            let rapl = rapl.elapsed_mut();

            let real = real.as_secs_f32();
            let user = user.as_secs_f32();
            let energy: f32 = rapl.values().sum();
            real_total += real;
            user_total += user;
            rapl_total += energy;

            if print_intermediate {
                println!("{},{},{},{},{},{}", size, pin_threads, mtd.num_threads, real, user, energy);
            }
        }
    }

    if !print_intermediate {
        println!("{},{},{}", real_total, user_total, rapl_total);
    }
}
