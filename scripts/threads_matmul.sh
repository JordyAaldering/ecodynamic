#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=threads_matmul.out

printf "size,threads,runtime,runtimesd,energy,energysd\n"

for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -t mt_pth scripts/matmul.sac -o matmul -DP=$size

    for threads in `seq 1 16`; do
        printf "$size,$threads,"
        ./matmul -mt_bind env -DSAC_NUM_SOCKETS=1 -DSAC_NUM_CORES=8 -DSAC_NUM_PUS=16 -mt $threads
    done
done

rm matmul
rm matmul.c
rm matmul.i
rm matmul.o
