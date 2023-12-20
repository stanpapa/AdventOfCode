use std::cmp::Ordering;

/// Calculate binomial coefficient (n choose k).
///
/// This function computes the binomial coefficient C(n, k)
///
/// Formula:
/// C(n, k) = n! / (k! * (n - k)!)
///
/// Reference:
/// [Binomial Coefficient - Wikipedia](https://en.wikipedia.org/wiki/Binomial_coefficient)
///
/// # Arguments
///
/// * `n` - The total number of items.
/// * `k` - The number of items to choose from `n`.
///
/// # Returns
///
/// Returns the binomial coefficient C(n, k).
pub trait BinomialCoefficient {
    type Output;

    fn binomial_coefficient(&self, k: Self) -> Self::Output;
}

/// Type specific implementation for the Factorial trait using a macro
macro_rules! binomial_impl {
    ($($t:ty),*) => {$(

impl BinomialCoefficient for $t {
    type Output = usize;

    fn binomial_coefficient(&self, k: Self) -> Self::Output {
        match self.cmp(&k) {
            Ordering::Greater => (0..k).fold(1, |acc, i| (acc * (*self  - i)) / (i + 1) ) as usize,
            Ordering::Equal => 1,
            Ordering::Less => panic!("k is larger than n!"),
        }
    }
}

    )*};
}

binomial_impl!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binon() {
        assert_eq!(0_usize.binomial_coefficient(0), 1);
        assert_eq!(1_usize.binomial_coefficient(1), 1);
        assert_eq!(2_usize.binomial_coefficient(1), 2);
        assert_eq!(2_usize.binomial_coefficient(2), 1);
        assert_eq!(3_usize.binomial_coefficient(1), 3);
        assert_eq!(3_usize.binomial_coefficient(2), 3);
        assert_eq!(3_usize.binomial_coefficient(3), 1);
        assert_eq!(4_usize.binomial_coefficient(1), 4);
        assert_eq!(4_usize.binomial_coefficient(2), 6);
        assert_eq!(4_usize.binomial_coefficient(3), 4);
    }
}
