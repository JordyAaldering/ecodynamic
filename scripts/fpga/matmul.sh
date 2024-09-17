#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=10:00:00
#SBATCH --output=log/fpga_matmul.out

make release

printf "dynamic,busy,threads,energy,runtime,usertime\n"

SIZE=500
ITER=20

for busy in `seq 0 4 32`; do
    for threads in `seq 1 32`; do
        printf "false,$busy,$threads,"
        numactl --interleave all ./target/release/examples/matmul $SIZE $(($ITER*$busy/2)) $busy true false &
        numactl --interleave all ./target/release/examples/matmul $SIZE $ITER $threads true true
        wait
    done
done

#for busy in `seq 0 4 32`; do
#    printf "true,$busy,$threads,"
#    ./target/release/busywork_f $busy numactl --interleave all ./target/release/examples/matmul $SIZE $ITER 32 false
#    printf "\n"
#done
