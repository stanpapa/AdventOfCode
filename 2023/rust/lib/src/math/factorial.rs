pub trait Factorial {
    /// The resulting type of the factorial functions
    type Output;

    /// Calculates the factorial of a number (n!)
    ///
    /// # Example
    /// ```
    /// // assert_eq!(5_u8.factorial(), 120);
    /// ```
    fn factorial(&self) -> Self::Output;
}

/// Type specific implementation for the Factorial trait using a macro
macro_rules! fact_unsigned_impl {
    ($($t:ty),*) => {$(

impl Factorial for $t {
    type Output = usize;

    fn factorial(&self) -> Self::Output {
        let n = *self as Self::Output;
        match self {
            0 | 1 => 1,
            _ => (1..=n).product(),
        }
    }
}

    )*};
}

fact_unsigned_impl!(u8, u16, u32, u64, usize);
