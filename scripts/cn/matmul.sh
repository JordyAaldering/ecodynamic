#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=log/cn_matmul.out

make release

printf "dynamic,busy,threads,energy,runtime,usertime\n"

SIZE=500
ITER=10

for busy in `seq 1 2 15`; do
    for threads in `seq 1 16`; do
        printf "false,$busy,$threads,"
        numactl --interleave all ./target/release/examples/matmul2 $SIZE $ITER $threads $busy
        wait
    done
done
