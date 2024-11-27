use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;
use rayon::prelude::*;

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn random(x: usize, y: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let data = (0..y).map(|_| {
            let mut row = vec![0.0; x];
            rng.fill(row.as_mut_slice());
            row
        }).collect();
        Self::new(data)
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut res = vec![vec![0.0; other.cols]; self.rows];

        res.par_iter_mut().enumerate().for_each(|(row_a, data)| {
            for col_b in 0..other.cols {
                for i in 0..self.cols {
                    data[col_b] += self.data[row_a][i] * other.data[i][col_b];
                }
            }
        });

        Self::new(res)
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <size> <max-threads>", args[0]);
        return;
    }

    let size: usize = args[1].parse().unwrap();
    let max_threads: i32 = args[2].parse().unwrap();

    let mut mtd = Mtd::runtime_controller(max_threads);

    loop {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

        let num_threads = mtd.num_threads() as usize;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads as usize)
            .build()
            .unwrap();

        let now = Instant::now();

        let _ = black_box(pool.install(|| x.mul(&y)));

        let sample = now.elapsed().as_secs_f32();
        mtd.update(sample);
    }
}
