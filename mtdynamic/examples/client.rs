use std::hint::black_box;

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

    /// { [i,j] -> sum({ [p] -> a[i,p] * b[p,j] }) }
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
    let x = Matrix::iota(500, 500);
    let y = Matrix::iota(500, 500);

    mtdynamic::MtdIterator::new(0..)
        .before(|d| println!("Received demand: {:?}", d))
        .after(|s| println!("Sending sample: {:?}", s))
        .for_each(|_| {
            black_box(x.mul(&y));
        });
}
