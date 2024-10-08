#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_matmul_mt.out

../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth -mt_bind simple matmul.sac -o matmul_mt -DP=1000 -DITER=20

# Warmup
./matmul -mt 16

for size in `seq 300 100 1500`; do
    ../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth_rt -mt_bind simple matmul.sac -o matmul_mt -DP=$size -DITER=20
    ./matmul_mt -mt 16
done

rm matmul_mt
rm matmul_mt.c
rm matmul_mt.i
rm matmul_mt.o
