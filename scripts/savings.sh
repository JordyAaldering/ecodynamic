#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=savings.out

ITER=200

cargo build --release --example matmul
cargo build --release --example matmul_mt

# Warmup
./target/release/examples/matmul 1000 50 16 true

printf "type,size,pin,runtime,usertime,energy\n"

#
# With thread pinning
#

for pin in true false; do
    for size in `seq 500 250 1500`; do
        printf "8,$size,$pin,"
        ./target/release/examples/matmul $size $ITER 8 $pin
        printf "12,$size,$pin,"
        ./target/release/examples/matmul $size $ITER 12 $pin
        printf "16,$size,$pin,"
        ./target/release/examples/matmul $size $ITER 16 $pin
        printf "mt,$size,$pin,"
        ./target/release/examples/matmul_mt $size $ITER 16 $pin
    done
done
