use rand::Rng;
use rayon::prelude::*;

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn random(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..y).map(|_| {
            let mut row = vec![0.0; x];
            rng.fill(row.as_mut_slice());
            row
        }).collect();
        Self::new(data)
    }

    pub fn mul(&self, other: &Matrix) -> Matrix {
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

pub fn threadpool(num_threads: usize, pin_threads: bool) -> rayon::ThreadPool {
    let mut builder = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads);

    if pin_threads {
        let cores = core_affinity::get_core_ids().unwrap();
        let max_threads = cores.len();
        assert!(num_threads <= max_threads);
        let thread_indices: Vec<usize> = (0..max_threads).step_by(2)
            .chain((1..max_threads).step_by(2)).collect();

        builder = builder.start_handler(move |idx| {
            let thread_idx = thread_indices[idx];
            let core_id = cores[thread_idx];
            assert!(core_affinity::set_for_current(core_id));
        });
    }

    builder.build().unwrap()
}
