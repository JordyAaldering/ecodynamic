#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=adapt_best.out

cargo build --release --example matmul

# Warmup
./target/release/examples/matmul 1000 20 16

printf "size,pin,threads,runtime,usertime,energy"

for size in `seq 500 50 1250`; do
    for threads in `seq 4 16`; do
        printf "$size,true,$threads,"
        ./target/release/examples/matmul $size 10 $threads true
        printf "$size,false,$threads,"
        ./target/release/examples/matmul $size 10 $threads false
    done
done
