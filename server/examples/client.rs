use std::hint::black_box;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Instant;

use controller::*;
use rapl_energy::{Probe, Rapl};
use rayon::prelude::*;

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

        data.par_iter_mut().enumerate().for_each(|(x, row)| {
            for y in 0..other.cols {
                for i in 0..self.cols {
                    row[y] += self.data[x][i] * other.data[i][y];
                }
            }
        });

        Self { rows: self.rows, cols: other.cols, data }
    }
}

/// First send a signal that we are at the start of a parallel region.
/// We don't actually care about the thread-count that we receive back.
pub fn region_start(stream: &mut UnixStream) -> (f32, Instant, Rapl) {
    stream.write_all(&Request {
        region_uid: 0,
        problem_size: 0
    }.to_bytes()).unwrap();

    let mut buf = [0u8; LocalDemand::SIZE];
    stream.read_exact(&mut buf).unwrap();
    let LocalDemand { threads_pct } = LocalDemand::from(buf);

    let rapl = Rapl::now(false).unwrap();
    let time = Instant::now();
    (threads_pct, time, rapl)
}

/// Signal an end of the region and send runtime and energy results.
pub fn region_stop(stream: &mut UnixStream, time: Instant, rapl: Rapl) {
    let runtime = time.elapsed();
    let energy = rapl.elapsed();

    let runtime = runtime.as_secs_f32();
    let energy = energy.values().sum();
    println!("Region end after {}s and {}J", runtime, energy);

    stream.write_all(&Sample {
        region_uid: 0,
        runtime: runtime,
        usertime: runtime,
        energy: energy,
    }.to_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let matrix_size: usize = args.get(1).map_or(500, |x| x.parse().unwrap());
    let max_threads: usize = args.get(2).map_or(16, |x| x.parse().unwrap());

    let mut stream = UnixStream::connect(MTD_LETTERBOX_PATH)?;

    let x = Matrix::iota(matrix_size, matrix_size);
    let y = Matrix::iota(matrix_size, matrix_size);

    loop {
        let (threads_pct, time, rapl) = region_start(&mut stream);
        let num_threads = ((threads_pct * max_threads as f32) as usize).max(1).min(max_threads);
        println!("Region start with {} threads", num_threads);

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap()
            .install(|| {
                black_box(x.mul(&y));
            });

        region_stop(&mut stream, time, rapl);
    }
}
