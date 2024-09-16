#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn127
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=log/cn_math.out

make release

printf "dynamic,busy,threads,energy,runtime,usertime\n"

REPEAT=10000
ITER=10

for busy in `seq 0 2 16`; do
    for threads in `seq 1 16`; do
        printf "false,$busy,$threads,"
        ./target/release/busywork_f 1 $busy ./target/release/examples/math $REPEAT $ITER $threads true
    done
done

for busy in `seq 0 2 16`; do
    printf "true,$busy,$threads,"
    ./target/release/busywork_f 1 $busy ./target/release/examples/math $REPEAT $ITER 16 false
done
