#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=adapt_matmul.out

printf "size,threads,runtime,runtimesd,energy,energysd\n"

# Static approaches
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -t mt_pth scripts/matmul.sac -o matmul -DP=$size

    printf "$size,1,"
    ./matmul -mt_bind env -DSAC_NUM_SOCKETS=1 -DSAC_NUM_CORES=8 -DSAC_NUM_PUS=16 -mt 1
    printf "$size,8,"
    ./matmul -mt_bind env -DSAC_NUM_SOCKETS=1 -DSAC_NUM_CORES=8 -DSAC_NUM_PUS=16 -mt 8
    printf "$size,12,"
    ./matmul -mt_bind env -DSAC_NUM_SOCKETS=1 -DSAC_NUM_CORES=8 -DSAC_NUM_PUS=16 -mt 12
    printf "$size,16,"
    ./matmul -mt_bind env -DSAC_NUM_SOCKETS=1 -DSAC_NUM_CORES=8 -DSAC_NUM_PUS=16 -mt 16
done

# Energy-based approach
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -t mt_pth_rt scripts/matmul.sac -o matmul -DP=$size

    printf "$size,mt,"
    numactl --interleave all ./matmul -mt 16
done

# Runtime-based approach
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -t mt_pth_rt -domtdrt scripts/matmul.sac -o matmul -DP=$size

    printf "$size,rt,"
    numactl --interleave all ./matmul -mt 16
done

rm matmul
rm matmul.c
rm matmul.i
rm matmul.o
