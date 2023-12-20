use crate::grid::coordinate::Coordinate;

use rayon::prelude::*;

pub fn area(polygon: &Vec<Coordinate>) -> usize {
    // find tiles enclosed in the main loop by using the shoelace formula and Pick's theorem
    // shoelace formula: area of simple polygon whose vertices are in the same xy-plane
    // A = 0.5 * sum_(i=1..=n) ( x_i * y_(i+1) - x_(i+1) * y_i )
    let length = polygon.len();
    ((1..=length)
        .into_par_iter()
        .map(|i| {
            polygon[i % length].x * polygon[(i + 1) % length].y
                - polygon[(i + 1) % length].x * polygon[i % length].y
        })
        .sum::<isize>()
        / 2)
    .abs() as usize
}

pub fn interior_points(polygon: &Vec<Coordinate>) -> usize {
    // Pick's theorem: area of a simple polygon with integer vertex coordinates
    // A = i + b / 2 - 1 => i = A - b / 2 + 1
    // A = area -> determined using shoelace formula
    // i = interior points -> this is what we want!!
    // b = boundary points -> number of points in the loop
    area(polygon) - polygon.len() / 2 + 1
}
