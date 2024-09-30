use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::MTDynamic;
use rand::Rng;
use rapl_energy::Rapl;
use rayon::prelude::*;

struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    fn random(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..y).map(|_| {
            let mut row = vec![0.0; x];
            rng.fill(row.as_mut_slice());
            row
        }).collect();
        Self::new(data)
    }

    fn mul(&self, other: &Matrix) -> Matrix {
        let mut res = vec![vec![0.0; other.cols]; self.rows];

        res.par_iter_mut().enumerate().for_each(|(row_a, data)| {
            for col_b in 0..other.cols {
                for i in 0..self.cols {
                    data[col_b] += self.data[row_a][i] * other.data[i][col_b];
                }
            }
        });

        Matrix::new(res)
    }
}

fn create_pool(num_threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

fn create_pool_pinned(num_threads: usize) -> rayon::ThreadPool {
    let cores = core_affinity::get_core_ids().unwrap();
    let max_threads = cores.len();
    assert!(num_threads <= max_threads);
    let thread_indices: Vec<usize> = (0..max_threads).step_by(2)
        .chain((1..max_threads).step_by(2)).collect();
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .start_handler(move |idx| {
            let thread_idx = thread_indices[idx];
            let core_id = cores[thread_idx];
            assert!(core_affinity::set_for_current(core_id));
        })
        .build()
        .unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <size> <iter> <threads> <pin_threads?>", args[0]);
        return;
    }

    let size: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let threads: i32 = args[3].parse().unwrap();
    let pin_threads: bool = args[4].parse().unwrap();

    let mut runtime: Vec<f64> = Vec::with_capacity(iter);
    let mut usertime: Vec<f64> = Vec::with_capacity(iter);
    let mut energy: Vec<f64> = Vec::with_capacity(iter);

    let mut rapl = Rapl::now().unwrap();

    let mut mtd = MTDynamic::new(threads, 20);
    let create_pool_fn = if pin_threads { create_pool } else { create_pool_pinned };
    let mut pool = create_pool_fn(threads as usize);

    let x = black_box(Matrix::random(size, size));
    let y = black_box(Matrix::random(size, size));

    for _ in 0..iter {
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
            pool = create_pool_fn(t);
        }
    }

    println!("{:.8},{:.8},{:.8}",
        runtime.into_iter().sum::<f64>() / iter as f64,
        usertime.into_iter().sum::<f64>() / iter as f64,
        energy.into_iter().sum::<f64>() / iter as f64,
    );
}
