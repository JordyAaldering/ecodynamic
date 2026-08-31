use std::hint::black_box;

use ecodynamic::{Connection, Region};

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn iota(rows: usize, cols: usize) -> Self {
        let data = (0..cols).map(|y| {
            (0..rows).map(|x| (x + y * rows) as f64).collect()
        }).collect();
        Self { rows, cols, data }
    }

    pub fn mul(&self, other: &Matrix) -> Matrix {
        let mut data = vec![vec![0.0; self.rows]; other.cols];
        data.iter_mut().enumerate().for_each(|(x, row)| {
            for y in 0..other.cols {
                for i in 0..self.cols {
                    row[y] += self.data[x][i] * other.data[i][y];
                }
            }
        });
        Self { rows: self.rows, cols: other.cols, data }
    }
}

fn main() {
    let max_threads = 4;
    let mut connection = Connection::connect(max_threads)
        .expect("Unable to connect to resource controller, is one running?");

    let x = Matrix::iota(400, 400);
    let y = Matrix::iota(400, 400);

    let mut region = Region::new(&mut connection, "matmul");

    for _ in 0..100 {
        let _demand = region.begin().unwrap();

        // Here we would use the demand to configure the number of threads

        let _ = black_box(x.mul(&y));

        let sample = region.end().unwrap();
        println!("{sample:?}");
    }
}
