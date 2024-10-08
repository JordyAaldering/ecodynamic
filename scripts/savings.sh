#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=savings.out

cargo build --release --example adapt_fast

# Warmup
stress --cpu 16 --timeout 30

printf "8,"
./target/release/examples/adapt_fast false 8
printf "12,"
./target/release/examples/adapt_fast false 12
printf "16,"
./target/release/examples/adapt_fast false 16
printf "mt,"
./target/release/examples/adapt_fast false

cargo build --release --example adapt_slow

printf "8,"
./target/release/examples/adapt_slow false 8
printf "12,"
./target/release/examples/adapt_slow false 12
printf "16,"
./target/release/examples/adapt_slow false 16
printf "mt,"
./target/release/examples/adapt_slow false
