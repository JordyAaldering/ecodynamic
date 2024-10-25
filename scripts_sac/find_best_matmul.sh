#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn127
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_find_best_matmul.out

printf "pin,size,threads,runtime,runtimesd,energy,energysd,name\n"

for pin in true false; do
    for size in `seq 500 100 1500`; do
        if [ $pin ]; then
            ../sac2c/build_r/sac2c_p -noprelude -t mt_pth -mt_bind simple scripts_sac/matmul.sac -o matmul -DP=$size
        else
            ../sac2c/build_r/sac2c_p -noprelude -t mt_pth scripts_sac/matmul.sac -o matmul -DP=$size
        fi

        for threads in `seq 1 16`; do
            printf "$pin,$size,$threads,"
            ./matmul -mt $threads
        done
    done
done

rm matmul
rm matmul.c
rm matmul.i
rm matmul.o
