#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn127
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_savings_relax.out

# With pinning
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -mt_bind simple -t mt_pth    relax.sac -o relax
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -mt_bind simple -t mt_pth_rt relax.sac -o relax_mt

# Warmup
stress --cpu 16 --timeout 30

printf "4,"
./relax -mt 4
printf "8,"
./relax -mt 8
printf "12,"
./relax -mt 12
printf "16,"
./relax -mt 16
printf "mt,"
./relax_mt -mt 16

# Without pinning
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth    relax.sac -o relax
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth_rt relax.sac -o relax_mt

# Warmup
stress --cpu 16 --timeout 30

printf "4,"
./relax -mt 4
printf "8,"
./relax -mt 8
printf "12,"
./relax -mt 12
printf "16,"
./relax -mt 16
printf "mt,"
./relax_mt -mt 16

# Cleanup
rm relax
rm relax.c
rm relax.i
rm relax.o

rm relax_mt
rm relax_mt.c
rm relax_mt.i
rm relax_mt.o
