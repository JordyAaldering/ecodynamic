/// We also need a benchmark that tests how quickly the controller converges to a new optimum,
/// after is has already converged to a previous one. This is important for workloads that have
/// multiple phases with different optimal powercaps, and for workloads that change their behavior over time.
///
/// 1. We pick some fixed runtime and energy curves, and iterate the controller for a fixed amount of time,
///    long enough for it to converge. This should probably include a bit of variability, because whether
///    the shift aligns with an evolution step or not can have a big impact.
/// 2. We then shift the curves to the provided ones, so that the optimal powercap changes,
///    and measure how long it takes for the controller to converge again.
fn main() {
    todo!();
}
