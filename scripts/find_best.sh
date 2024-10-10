#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=find_best.out

ITER=50

cargo build --release --example matmul

# Warmup
stress --cpu 16 --timeout 30

printf "pin,size,threads,runtime,runtimestd,usertime,usertimestd,energy,energystd\n"

for pin in true false; do
    for size in `seq 500 250 2500`; do
        for threads in `seq 4 16`; do
            printf "$pin,$size,$threads,"
            ./target/release/examples/matmul $size $ITER $threads $pin
        done
    done
done
