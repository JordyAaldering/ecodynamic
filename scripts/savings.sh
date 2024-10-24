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

printf "8 threads:\n"
./target/release/examples/savings 8
printf "12 threads:\n"
./target/release/examples/savings 12
printf "16 threads:\n"
./target/release/examples/savings 16
printf "Dynamic adaptation:\n"
./target/release/examples/savings
