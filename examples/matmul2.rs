use std::{hint::black_box, thread, time::Instant};

use cpu_time::ProcessTime;
use rand::Rng;
use rapl_energy::Rapl;
use rayon::prelude::*;

#[derive(Clone)]
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

fn create_random_matrix(x: usize, y: usize) -> Matrix {
    let mut rng = rand::thread_rng();
    let data = (0..y).map(|_| {
        let mut row = vec![0.0; x];
        rng.fill(row.as_mut_slice());
        row
    }).collect();
    Matrix::new(data)
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
        eprintln!("Usage: {} <size> <iter> <self_threads> <busy_threads>", args[0]);
        return;
    }

    let size: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();
    let self_threads: i32 = args[3].parse().unwrap();
    let busy_threads: i32 = args[4].parse().unwrap();

    let x = black_box(create_random_matrix(size, size));
    let y = black_box(create_random_matrix(size, size));

    let mut energies: Vec<f64> = Vec::with_capacity(iter);
    let mut reals: Vec<f64> = Vec::with_capacity(iter);
    let mut users: Vec<f64> = Vec::with_capacity(iter);

    let mut rapl = Rapl::now().unwrap();

    let self_pool = create_pool(self_threads as usize);

    for _ in 0..iter {
        let busy_pool = create_pool(busy_threads as usize);
        let x_busy = x.clone();
        let y_busy = y.clone();

        let _ = rapl.elapsed_mut();
        let user = ProcessTime::now();
        let real = Instant::now();

        let handle = thread::spawn(move || {
            if busy_threads > 0 {
                busy_pool.install(|| {
                    let _ = black_box(x_busy.clone().mul(&y_busy));
                });
            }
        });

        self_pool.install(|| {
            let _ = black_box(x.mul(&y));
        });

        handle.join().unwrap();

        let real = real.elapsed();
        let user = user.elapsed();
        let energy = rapl.elapsed_mut().values().sum();
        energies.push(energy);

        let real = real.as_secs_f64();
        let user = user.as_secs_f64();
        reals.push(real);
        users.push(user);
    }

    let energy_avg = energies.into_iter().sum::<f64>() / iter as f64;
    let real_avg = reals.into_iter().sum::<f64>() / iter as f64;
    let user_avg = users.into_iter().sum::<f64>() / iter as f64;
    print!("{:.8},{:.8},{:.8}", energy_avg, real_avg, user_avg);
}
