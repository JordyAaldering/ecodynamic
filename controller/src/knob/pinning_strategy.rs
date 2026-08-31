/// Strategy used to pin the threads of a chromosome.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PinningStrategy {
    /// Let the OS scheduler decide thread placement.
    #[default]
    Free,
}
