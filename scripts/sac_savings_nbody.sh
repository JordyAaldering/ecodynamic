#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn126
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_savings_nbody.out

# With pinning
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -mt_bind simple -t mt_pth    nbody.sac -o nbody
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -mt_bind simple -t mt_pth_rt nbody.sac -o nbody_mt

# Warmup
stress --cpu 16 --timeout 30

printf "4,"
./nbody -mt 4
printf "8,"
./nbody -mt 8
printf "12,"
./nbody -mt 12
printf "16,"
./nbody -mt 16
printf "mt,"
./nbody_mt -mt 16

# Without pinning
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth    nbody.sac -o nbody
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth_rt nbody.sac -o nbody_mt

# Warmup
stress --cpu 16 --timeout 30

printf "4,"
./nbody -mt 4
printf "8,"
./nbody -mt 8
printf "12,"
./nbody -mt 12
printf "16,"
./nbody -mt 16
printf "mt,"
./nbody_mt -mt 16

# Cleanup
rm nbody
rm nbody.c
rm nbody.i
rm nbody.o

rm nbody_mt
rm nbody_mt.c
rm nbody_mt.i
rm nbody_mt.o
