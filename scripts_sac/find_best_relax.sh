#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn125
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_find_best_relax.out

printf "size,threads,runtime,runtimesd,energy,energysd,name\n"

for size in 100 1000 10000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 25 -t mt_pth -mt_bind simple scripts_sac/relax.sac -o relax -DP=$size

    for threads in `seq 1 16`; do
        printf "$size,$threads,"
        ./relax -mt $threads
    done
done

rm relax
rm relax.c
rm relax.i
rm relax.o
