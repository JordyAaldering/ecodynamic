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
    let print_intermediate: bool = args[1].parse().unwrap();
    let mut num_threads = 16;
    let mut dynamic = true;
    if args.len() > 2 {
        num_threads = args[2].parse().unwrap();
        dynamic = true;
    }

    let mut mtd = MTDynamic::new(16, 10);
    let mut rapl = Rapl::now().unwrap();

    const CYCLES: [(usize, bool); 20] = [
        (500, false),
        (750, false),
        (1000, false),
        (1250, false),
        (1500, false),
        (500, true),
        (750, true),
        (1000, true),
        (1250, true),
        (1500, true),
        (500, false),
        (750, false),
        (1000, false),
        (1250, false),
        (1500, false),
        (500, true),
        (750, true),
        (1000, true),
        (1250, true),
        (1500, true),
    ];

    if print_intermediate {
        println!("size,pin,threads,runtime,usertime,energy");
    }

    let mut real_total = 0.0;
    let mut user_total = 0.0;
    let mut rapl_total = 0.0;

    for (size, pin) in CYCLES {
        let mut pool = if pin { create_pool_pinned(num_threads) } else { create_pool(num_threads) };

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
                println!("{},{},{},{:.8},{:.8},{:.8}", size, pin, num_threads, real, user, rapl);
            }

            if dynamic {
                mtd.update("parallel", real, user, rapl);
                num_threads = mtd.num_threads("parallel") as usize;
                if pool.current_num_threads() != num_threads {
                    pool = if pin { create_pool_pinned(num_threads) } else { create_pool(num_threads) };
                }
            }
        }
    }

    if !print_intermediate {
        println!("{},{},{}", real_total, user_total, rapl_total);
    }
}
