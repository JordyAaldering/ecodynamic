#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=observations.out

cargo build --release --example matmul

# Warmup
./target/release/examples/matmul 1000 10 16

for size in `seq 300 100 1500`; do
    for threads in `seq 1 16`; do
        printf "$size,$threads,"
        ./target/release/examples/matmul $size $(($threads+4)) $threads
    done
done
