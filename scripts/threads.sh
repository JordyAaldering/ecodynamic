#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=6:00:00
#SBATCH --output=log/threads.out

make release

printf "busy,threads,energy,runtime,usertime\n"

LEN=10000000
ITER=500

for busy in `seq 0 4 32`; do
    for threads in `seq 1 32`; do
        printf "$busy,$threads,"
        ./target/release/busywork_f 1 $busy ./target/release/examples/parallel $LEN $ITER $threads true
    done
done
