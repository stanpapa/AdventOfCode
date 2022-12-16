use crate::grid::coordinate::Coordinate;

pub trait ManhattanDistance {
    fn manhattan_distance(&self, other: &Self) -> usize;
}

impl ManhattanDistance for Coordinate {
    fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate() {
        assert_eq!(
            Coordinate::new(2, 18).manhattan_distance(&Coordinate::new(-2, 15)),
            7
        );
    }
}
