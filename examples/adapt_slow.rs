#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::MtdBuilder;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let print_intermediate: bool = args[1].parse().unwrap();
    let mut num_threads = 16;
    let mut dynamic = true;
    if args.len() > 2 {
        num_threads = args[2].parse().unwrap();
        dynamic = true;
    }

    let mut mtd = MtdBuilder::new(16).build();
    let mut rapl = Rapl::now().unwrap();

    const CYCLES: [(usize, bool); 20] = [
        (800, false),
        (850, false),
        (900, false),
        (950, false),
        (1000, false),
        (1050, false),
        (1100, false),
        (1150, false),
        (1200, false),
        (1250, false),
        (1250, true),
        (1200, true),
        (1150, true),
        (1100, true),
        (1050, true),
        (1000, true),
        (950, true),
        (900, true),
        (850, true),
        (800, true),
    ];

    if print_intermediate {
        println!("size,pin,threads,runtime,usertime,energy");
    }

    let mut real_total = 0.0;
    let mut user_total = 0.0;
    let mut rapl_total = 0.0;

    for (size, pin_threads) in CYCLES {
        let mut pool = threadpool(num_threads, pin_threads);

        for _ in 0..200 {
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
            real_total += real;
            user_total += user;
            rapl_total += rapl;

            if print_intermediate {
                let num_threads_f64 = mtd.controllers["parallel"].1.num_threads;
                println!("{},{},{},{:.8},{:.8},{:.8}", size, pin_threads, num_threads_f64, real, user, rapl);
            }

            if dynamic {
                mtd.update("parallel", real, user, rapl);
                num_threads = mtd.num_threads("parallel") as usize;
                if pool.current_num_threads() != num_threads {
                    pool = threadpool(num_threads, pin_threads);
                }
            }
        }
    }

    if !print_intermediate {
        println!("{},{},{}", real_total, user_total, rapl_total);
    }
}
