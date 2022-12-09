use crate::grid::coordinate::Coordinate;

pub trait ChebyshevDistance {
    fn chebyshev_distance(&self, other: &Self) -> usize;
}

impl ChebyshevDistance for Coordinate {
    fn chebyshev_distance(&self, other: &Self) -> usize {
        std::cmp::max((self.x - other.x).abs(), (self.y - other.y).abs()) as usize
    }
}
