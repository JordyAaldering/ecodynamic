#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_matmul.out

../sac2c/build_r/sac2c_p -noprelude -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=1000 -DITER=20

# Warmup
./matmul -mt 16

for size in `seq 300 100 1500`; do
    ../sac2c/build_r/sac2c_p -noprelude -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=$size -DITER=20

    for threads in `seq 1 16`; do
        printf "$size,$threads,"
        ./matmul -mt $threads
    done
done

rm matmul
rm matmul.c
rm matmul.i
rm matmul.o
