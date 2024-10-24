#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=savings.out

cargo build --release --example savings

# Warmup
stress --cpu 16 --timeout 30

printf "08,"
./target/release/examples/savings 8
printf "12,"
./target/release/examples/savings 12
printf "16,"
./target/release/examples/savings 16
printf "mt,"
./target/release/examples/savings
