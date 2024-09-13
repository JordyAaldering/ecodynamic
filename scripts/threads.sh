#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=1:00:00
#SBATCH --output=log/threads.out

cargo build -q --release --example parallel

printf "threads,energy,runtime,user-pct\n"

for i in `seq 1 32`; do
    printf "$i,"
    ./target/release/examples/parallel $i true
done
