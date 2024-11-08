#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=compare.out

ITER=200

cargo build --release --lib --examples

# Warmup
stress --cpu 16 --timeout 30

printf "type,size,pin,runtime,runtimestd,usertime,usertimestd,energy,energystd\n"

#
# Without thread pinning
#

# 500 threads not pinned, optimum = 16 threads
printf "oracle,500,false,"
./target/release/examples/matmul 500 $ITER 16 false "fixed"
printf "mt,500,false,"
./target/release/examples/matmul 500 $ITER 16 false "energy"
printf "rt,500,false,"
./target/release/examples/matmul 500 $ITER 16 false "runtime"

# 750 threads not pinned, optimum = 16 threads
printf "oracle,750,false,"
./target/release/examples/matmul 750 $ITER 16 false "fixed"
printf "mt,750,false,"
./target/release/examples/matmul 750 $ITER 16 false "energy"
printf "rt,750,false,"
./target/release/examples/matmul 750 $ITER 16 false "runtime"

# 1000 threads not pinned, optimum = 16 threads
printf "oracle,1000,false,"
./target/release/examples/matmul 1000 $ITER 16 false "fixed"
printf "mt,1000,false,"
./target/release/examples/matmul 1000 $ITER 16 false "energy"
printf "rt,1000,false,"
./target/release/examples/matmul 1000 $ITER 16 false "runtime"

# 1250 threads not pinned, optimum = 8 threads
printf "oracle,1250,false,"
./target/release/examples/matmul 1250 $ITER 8 false "fixed"
printf "mt,1250,false,"
./target/release/examples/matmul 1250 $ITER 16 false "energy"
printf "rt,1250,false,"
./target/release/examples/matmul 1250 $ITER 16 false "runtime"

# 1500 threads not pinned, optimum = 8 threads
printf "oracle,1500,false,"
./target/release/examples/matmul 1500 $ITER 8 false "fixed"
printf "mt,1500,false,"
./target/release/examples/matmul 1500 $ITER 16 false "energy"
printf "rt,1500,false,"
./target/release/examples/matmul 1500 $ITER 16 false "runtime"

# 1750 threads not pinned, optimum = 8 threads
printf "oracle,1750,false,"
./target/release/examples/matmul 1750 $ITER 8 false "fixed"
printf "mt,1750,false,"
./target/release/examples/matmul 1750 $ITER 16 false "energy"
printf "rt,1750,false,"
./target/release/examples/matmul 1750 $ITER 16 false "runtime"

#
# With thread pinning
#

# 500 threads pinned, optimum = 16 threads
printf "oracle,500,true,"
./target/release/examples/matmul 500 $ITER 16 true "fixed"
printf "mt,500,true,"
./target/release/examples/matmul 500 $ITER 16 true "energy"
printf "rt,500,true,"
./target/release/examples/matmul 500 $ITER 16 true "runtime"

# 750 threads pinned, optimum = 16 threads
printf "oracle,750,true,"
./target/release/examples/matmul 750 $ITER 16 true "fixed"
printf "mt,750,true,"
./target/release/examples/matmul 750 $ITER 16 true "energy"
printf "rt,750,true,"
./target/release/examples/matmul 750 $ITER 16 true "runtime"

# 1000 threads pinned, optimum = 16 threads
printf "oracle,1000,true,"
./target/release/examples/matmul 1000 $ITER 16 true "fixed"
printf "mt,1000,true,"
./target/release/examples/matmul 1000 $ITER 16 true "energy"
printf "rt,1000,true,"
./target/release/examples/matmul 1000 $ITER 16 true "runtime"

# 1250 threads pinned, optimum = 12 threads
printf "oracle,1250,true,"
./target/release/examples/matmul 1250 $ITER 12 true "fixed"
printf "mt,1250,true,"
./target/release/examples/matmul 1250 $ITER 16 true "energy"
printf "rt,1250,true,"
./target/release/examples/matmul 1250 $ITER 16 true "runtime"

# 1500 threads pinned, optimum = 12 threads
printf "oracle,1500,true,"
./target/release/examples/matmul 1500 $ITER 12 true "fixed"
printf "mt,1500,true,"
./target/release/examples/matmul 1500 $ITER 16 true "energy"
printf "rt,1500,true,"
./target/release/examples/matmul 1500 $ITER 16 true "runtime"

# 1750 threads pinned, optimum = 12 threads
printf "oracle,1750,true,"
./target/release/examples/matmul 1750 $ITER 12 true "fixed"
printf "mt,1750,true,"
./target/release/examples/matmul 1750 $ITER 16 true "energy"
printf "rt,1750,true,"
./target/release/examples/matmul 1750 $ITER 16 true "runtime"
