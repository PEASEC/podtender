/// This trait is intended to be implemented by parameter types to showcase a possible usage.
#[cfg(any(test, feature = "examples"))]
pub trait ExampleValues {
    fn example() -> Self;
}
