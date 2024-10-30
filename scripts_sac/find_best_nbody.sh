#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn126
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_find_best_nbody.out

printf "pin,size,threads,runtime,runtimesd,energy,energysd\n"

for pin in true; do
    for size in 1000 5000 10000 25000 50000; do
        if [ $pin ]; then
            ../sac2c/build_r/sac2c_p -noprelude -t mt_pth -mt_bind simple scripts_sac/nbody.sac -o nbody -DP=$size
        else
            ../sac2c/build_r/sac2c_p -noprelude -t mt_pth scripts_sac/nbody.sac -o nbody -DP=$size
        fi

        for threads in `seq 1 16`; do
            printf "$pin,$size,$threads,"
            ./nbody -mt $threads
        done
    done
done

rm nbody
rm nbody.c
rm nbody.i
rm nbody.o
