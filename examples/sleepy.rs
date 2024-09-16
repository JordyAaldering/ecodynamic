use std::{hint::black_box, thread::sleep, time::{Duration, Instant}};

use cpu_time::ProcessTime;
use mtdynamic::MTDynamic;
use rapl_energy::Rapl;
use rayon::prelude::*;

fn parallel(v: &mut Vec<f64>, repeat: usize, dur: Duration) {
    v.par_iter_mut().for_each(|x| {
        sleep(dur);
        for _ in 0..repeat {
            *x = f64::sqrt(black_box(x.ln() * x.ln()));
        }
    });
}

pub fn create_pool(num_threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
       .num_threads(num_threads)
       .build()
       .unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <repeat> <iter> <max_threads> <threads_fixed>", args[0]);
        return;
    }

    let repeat: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let max_threads: i32 = args[3].parse().unwrap();
    let threads_fixed: bool = args[4].parse().unwrap();

    let mut v: Vec<f64> = (0..1024).map(|x| x as f64).collect();

    let mut energies: Vec<f64> = Vec::with_capacity(iter);
    let mut reals: Vec<f64> = Vec::with_capacity(iter);
    let mut users: Vec<f64> = Vec::with_capacity(iter);

    let mut mtd = MTDynamic::new(max_threads, 10);
    let mut rapl = Rapl::now().unwrap();

    let mut num_threads = max_threads as usize;
    let mut pool = create_pool(num_threads);
    for _ in 0..iter {
        let _ = rapl.elapsed_mut();
        let user = ProcessTime::now();
        let real = Instant::now();

        pool.install(|| {
            parallel(&mut v, repeat, Duration::from_micros(200 * num_threads as u64));
        });

        let real = real.elapsed();
        let user = user.elapsed();
        let energy = rapl.elapsed_mut().values().sum();
        energies.push(energy);

        let real = real.as_secs_f64();
        let user = user.as_secs_f64();
        reals.push(real);
        users.push(user);

        if !threads_fixed {
            mtd.update("parallel", real, user, energy);
            let t = mtd.num_threads("parallel") as usize;
            if pool.current_num_threads() != t {
                pool = create_pool(t);
                num_threads = t;
            }
        }
    }

    let energy_avg = energies.into_iter().sum::<f64>() / iter as f64;
    let real_avg = reals.into_iter().sum::<f64>() / iter as f64;
    let user_avg = users.into_iter().sum::<f64>() / iter as f64;
    println!("{:.8},{:.8},{:.8}", energy_avg, real_avg, user_avg);
}
