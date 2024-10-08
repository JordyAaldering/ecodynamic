#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_observe.out

ITER=20

printf "pin,size,threads,,energy,runtime,usertime\n"

# With pinning
for size in `seq 500 50 1250`; do
    ../sac2c/build_r/sac2c_p -noprelude -specmode AKD -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=$size -DITER=$ITER
    for threads in `seq 1 16`; do
        printf "true,$size,$threads,"
        ./matmul -mt $threads
    done
done
for size in `seq 1500 250 2500`; do
    ../sac2c/build_r/sac2c_p -noprelude -specmode AKD -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=$size -DITER=$ITER
    for threads in `seq 1 16`; do
        printf "true,$size,$threads,"
        ./matmul -mt $threads
    done
done

# Without pinning
for size in `seq 500 50 1250`; do
    ../sac2c/build_r/sac2c_p -noprelude -specmode AKD -t mt_pth matmul.sac -o matmul -DP=$size -DITER=$ITER
    for threads in `seq 1 16`; do
        printf "false,$size,$threads,"
        ./matmul -mt $threads
    done
done
for size in `seq 1500 250 2500`; do
    ../sac2c/build_r/sac2c_p -noprelude -specmode AKD -t mt_pth matmul.sac -o matmul -DP=$size -DITER=$ITER
    for threads in `seq 1 16`; do
        printf "false,$size,$threads,"
        ./matmul -mt $threads
    done
done

rm matmul_mt
rm matmul_mt.c
rm matmul_mt.i
rm matmul_mt.o
