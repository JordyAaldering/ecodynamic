use std::{hint::black_box, thread, time::Instant};

use cpu_time::ProcessTime;
use rapl_energy::Rapl;
use rayon::prelude::*;

fn parallel(v: &mut Vec<f64>, repeat: usize) {
    v.par_iter_mut().for_each(|x| {
        for _ in 0..repeat {
            *x = f64::sqrt(black_box(x.ln() * x.ln()));
        }
    });
}

fn create_pool(num_threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
       .num_threads(num_threads)
       .build()
       .unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <repeat> <iter> <self_threads> <busy_threads>", args[0]);
        return;
    }

    let repeat: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let self_threads: i32 = args[3].parse().unwrap();
    let busy_threads: i32 = args[4].parse().unwrap();

    let mut v: Vec<f64> = (0..1024).map(|x| x as f64).collect();

    let mut energies: Vec<f64> = Vec::with_capacity(iter);
    let mut reals: Vec<f64> = Vec::with_capacity(iter);
    let mut users: Vec<f64> = Vec::with_capacity(iter);

    let mut rapl = Rapl::now().unwrap();

    let self_pool = create_pool(self_threads as usize);

    for _ in 0..iter {
        let busy_pool = create_pool(busy_threads as usize);
        let mut v2 = v.clone();

        let _ = rapl.elapsed_mut();
        let user = ProcessTime::now();
        let real = Instant::now();

        let handle = thread::spawn(move || {
            if busy_threads > 0 {
                busy_pool.install(|| {
                    parallel(&mut v2, repeat);
                });
            }
        });

        self_pool.install(|| {
            parallel(&mut v, repeat);
        });

        handle.join().unwrap();

        let real = real.elapsed();
        let user = user.elapsed();
        let energy = rapl.elapsed_mut().values().sum();
        energies.push(energy);
        reals.push(real.as_secs_f64());
        users.push(user.as_secs_f64());
    }

    let energy_avg = energies.into_iter().sum::<f64>() / iter as f64;
    let real_avg = reals.into_iter().sum::<f64>() / iter as f64;
    let user_avg = users.into_iter().sum::<f64>() / iter as f64;
    println!("{:.8},{:.8},{:.8}", energy_avg, real_avg, user_avg);
}
