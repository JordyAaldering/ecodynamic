#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=1:00:00
#SBATCH --output=log/threads.out

cargo build -q --release --example parallel

printf "threads,energy,runtime,usertime\n"

for i in `seq 1 16`; do
    printf "$i,"
    ./target/release/examples/parallel 10000000 2000 $i true
done
