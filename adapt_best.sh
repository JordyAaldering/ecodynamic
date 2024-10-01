#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=cn128_matmul_rust.out

cargo build --release --example matmul

# Warmup
./target/release/examples/matmul 1000 20 16

for size in `seq 800 50 1250`; do
    for threads in `seq 4 16`; do
        printf "$size,true,$threads,"
        ./target/release/examples/matmul $size 20 $threads true
        printf "$size,false,$threads,"
        ./target/release/examples/matmul $size 20 $threads false
    done
done
