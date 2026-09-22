use std::{hint::black_box, io};

/// Some long-running, repeated task we want to control.
fn factorial(x: u128) -> u128 {
    (1..x).fold(1, u128::saturating_mul)
}

fn main() -> io::Result<()> {
    // This would normally be derived, for example from an environment variable
    let max_threads = 4;

    let mut connection = ecodynamic::connect(max_threads)?;

    let mut task = ecodynamic::task(&mut connection, "factorial");

    loop {
        let demand = task.begin()?;
        // Here we would use the demand to configure the number of threads
        println!("Demand: {demand:?}");

        let _ = black_box(factorial(10000000));

        let sample = task.end()?;
        // The sample is returned for debugging purposes, but otherwise no further action is needed
        println!("Sample: {sample:?}");
    }
}
