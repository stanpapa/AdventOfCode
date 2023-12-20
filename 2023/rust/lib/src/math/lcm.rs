use crate::math::gcd::gcd;

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
