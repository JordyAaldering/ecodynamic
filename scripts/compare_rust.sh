#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=compare_rust.out

cargo build --release --example compare

printf "threads,size,pin,runtime,runtimesd,energy,energysd\n"

for size in 500 1000 1500; do
    numactl --interleave all ./target/release/examples/compare $size 1  s
    numactl --interleave all ./target/release/examples/compare $size 8  s
    numactl --interleave all ./target/release/examples/compare $size 12 s
    numactl --interleave all ./target/release/examples/compare $size 16 s
    numactl --interleave all ./target/release/examples/compare $size 16 e
    numactl --interleave all ./target/release/examples/compare $size 16 r
done
