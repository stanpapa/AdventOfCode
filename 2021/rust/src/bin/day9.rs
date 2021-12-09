use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use aoc_2021_rust::utils::{coordinate::Coordinate, input};

static DIRECTIONS: [Coordinate; 4] = [
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 0, y: -1 },
    Coordinate { x: -1, y: 0 },
];

#[derive(Clone)]
struct HeightMap {
    map: HashMap<Coordinate, u8>,
}

impl HeightMap {
    fn new(s: String) -> Self {
        let mut map = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, height) in line.chars().enumerate() {
                map.insert(
                    Coordinate {
                        x: x as i32,
                        y: y as i32,
                    },
                    height.to_digit(10).unwrap() as u8,
                );
            }
        }

        Self { map }
    }

    // check if current point is a low point
    fn check_point(&self, point: Coordinate) -> bool {
        for direction in DIRECTIONS {
            let moved = point + direction;
            if self.map.contains_key(&moved) {
                if self.map.get(&moved) <= self.map.get(&point) {
                    return false;
                }
            }
        }

        true
    }

    fn find_low_points(&self) -> Vec<Coordinate> {
        self.map
            .iter()
            .filter(|(&pos, _height)| self.check_point(pos))
            .map(|(&pos, _height)| pos)
            .collect()
    }

    // risk level is height of low point + 1
    fn risk_level(&self, point: Coordinate) -> u8 {
        self.map.get(&point).unwrap() + 1
    }
}

// sum the risk levels of all low points
fn part_1(map: &HeightMap) -> u32 {
    map.find_low_points()
        .iter()
        .map(|low| map.risk_level(*low) as u32)
        .sum()
}

// recursive function to traverse basins and return their size. Depth-first search
fn remove_point(point: Coordinate, grid: &mut HashSet<Coordinate>) -> usize {
    // if point not on the grid, return 0
    if !grid.remove(&point) {
        return 0;
    }

    // add 1 to basin size, since the point was there and continue the search
    1 + [
        point + DIRECTIONS[0],
        point + DIRECTIONS[1],
        point + DIRECTIONS[2],
        point + DIRECTIONS[3],
    ]
    .iter()
    .map(|&neighbour| remove_point(neighbour, grid))
    .sum::<usize>()
}

// find 3 largest basins basins and multiply their sizes
fn part_2(map: &HeightMap) -> usize {
    // height is not important, nor do we care about 9's
    let mut points = map
        .map
        .iter()
        .filter(|&(_point, height)| height.clone() != 9)
        .map(|(point, _height)| *point)
        .collect::<HashSet<_>>();

    // traverse grid and calculate basin sizes
    let mut basin_sizes = Vec::new();
    while let Some(&point) = points.iter().next() {
        basin_sizes.push(remove_point(point, &mut points));
    }

    // sort by basin size and return the product of the largest 3
    basin_sizes.iter().sorted().rev().take(3).product()
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_string()?;
    let map = HeightMap::new(input);

    println!("Part 1: {}", part_1(&map));
    println!("Part 2: {}", part_2(&map));

    Ok(())
}
