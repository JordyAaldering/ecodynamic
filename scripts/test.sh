#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=1:00:00
#SBATCH --output=log/test.out

./target/release/busywork 12 ./target/release/examples/parallel 50000000 5000 16 false
