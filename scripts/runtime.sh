#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=oracle.out

ITER=200

cargo build --release --example matmul
cargo build --release --example matmul_rt

# Warmup
./target/release/examples/matmul 1000 50 16 true

printf "type,size,pin,runtime,usertime,energy\n"

# 500 threads pinned, optimum = 16 threads
printf "rt,500,true,"
./target/release/examples/matmul_rt 500 $ITER 16 true

# 500 threads not pinned, optimum = 16 threads
printf "rt,500,false,"
./target/release/examples/matmul_rt 500 $ITER 16 false



# 750 threads pinned, optimum = 16 threads
printf "rt,750,true,"
./target/release/examples/matmul_rt 750 $ITER 16 true

# 750 threads not pinned, optimum = 16 threads
printf "rt,750,false,"
./target/release/examples/matmul_rt 750 $ITER 16 false



# 1000 threads pinned, optimum = 16 threads
printf "rt,1000,true,"
./target/release/examples/matmul_rt 1000 $ITER 16 true

# 1000 threads not pinned, optimum = 14 threads
printf "rt,1000,false,"
./target/release/examples/matmul_rt 1000 $ITER 16 false



# 1250 threads pinned, optimum = 12 threads
printf "rt,1250,true,"
./target/release/examples/matmul_rt 1250 $ITER 16 true

# 1250 threads not pinned, optimum = 8 threads
printf "rt,1250,false,"
./target/release/examples/matmul_rt 1250 $ITER 16 false



# 1500 threads pinned, optimum = 12 threads
printf "rt,1500,true,"
./target/release/examples/matmul_rt 1500 $ITER 16 true

# 1500 threads not pinned, optimum = 8 threads
printf "rt,1500,false,"
./target/release/examples/matmul_rt 1500 $ITER 16 false
