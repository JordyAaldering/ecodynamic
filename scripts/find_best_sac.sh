#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=find_best_sac.out

ITER=50

cargo build --release --example matmul

# Warmup
stress --cpu 16 --timeout 30

printf "pin,size,threads,runtime,runtimestd,usertime,usertimestd,energy,energystd\n"

for pin in true false; do
    for size in `seq 500 250 2000`; do
        if [ $pin ]; then
            ../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=$SIZE -DITER=$ITER
        else
            ../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -t mt_pth matmul.sac -o matmul -DP=$SIZE -DITER=$ITER
        fi

        for threads in `seq 1 16`; do
            printf "$pin,$size,$threads,"
            ./target/release/examples/matmul $size $ITER $threads $pin
        done
    done
done
