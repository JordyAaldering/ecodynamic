#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=10:00:00
#SBATCH --output=log/fpga_math.out

make release

printf "dynamic,busy,threads,energy,runtime,usertime\n"

REPEAT=10000
ITER=10

for busy in `seq 1 4 32`; do
    for threads in `seq 1 32`; do
        printf "false,$busy,$threads,"
        numactl --interleave all ./target/release/examples/math2 $REPEAT $ITER $threads $busy
    done
done
