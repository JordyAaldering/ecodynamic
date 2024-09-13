#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=6:00:00
#SBATCH --output=log/threads.out

cargo build -q --release --example parallel

printf "busy,threads,energy,runtime,usertime\n"

LEN=10000000
ITER=1000

for busy in 0 2 4 8 16 32; do
    for threads in `seq 1 32`; do
        printf "$busy,$threads,"
        ./target/release/examples/parallel $LEN $ITER $threads true $busy
    done
done
