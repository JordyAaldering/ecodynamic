#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=adapt_nbody.out

printf "size,threads,runtime,runtimesd,energy,energysd\n"

# Static approaches
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -t mt_pth scripts/nbody.sac -o nbody -DP=$size

    printf "$size,1,"
    numactl --interleave all ./nbody -mt 1
    printf "$size,8,"
    numactl --interleave all ./nbody -mt 8
    printf "$size,12,"
    numactl --interleave all ./nbody -mt 12
    printf "$size,14,"
    numactl --interleave all ./nbody -mt 14
    printf "$size,16,"
    numactl --interleave all ./nbody -mt 16
done

# Energy-based approach
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -t mt_pth_rt scripts/nbody.sac -o nbody -DP=$size

    printf "$size,mt,"
    numactl --interleave all ./nbody -mt 16
done

# Runtime-based approach
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -t mt_pth_rt -domtdrt scripts/nbody.sac -o nbody -DP=$size

    printf "$size,rt,"
    numactl --interleave all ./nbody -mt 16
done

rm nbody
rm nbody.c
rm nbody.i
rm nbody.o
