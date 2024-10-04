#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=find_best.out

cargo build --release --example matmul

# Warmup
./target/release/examples/matmul 1000 50 16

printf "size,pin,threads,runtime,usertime,energy\n"

for pin in true false; do
    for size in `seq 500 50 1250`; do
        for threads in `seq 4 16`; do
            printf "$size,$pin,$threads,"
            ./target/release/examples/matmul $size 20 $threads $pin
        done
    done

    for size in `seq 1500 250 2500`; do
        for threads in `seq 4 16`; do
            printf "$size,$pin,$threads,"
            ./target/release/examples/matmul $size 20 $threads $pin
        done
    done
done
