extern crate rayon;
use rayon::prelude::*;

extern crate regex;
use regex::Regex;

use std::{collections::HashSet, error::Error, ops::ControlFlow};

use aoc_lib::{
    grid::{coordinate::Coordinate, manhattan_distance::ManhattanDistance},
    io::input::Input,
};

fn parse(s: &str) -> (Coordinate, Coordinate) {
    // construct regex pattern to match input line
    let input_regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    // collect matches in vector and convert them to isize
    let captures = input_regex.captures(s).map(|captures| {
        captures
            .iter()
            .skip(1) // skip full match
            .map(|c| c.unwrap().as_str().parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    });

    // return sensor beacon Coordinate pair
    match captures {
        Some(c) => (Coordinate::new(c[0], c[1]), Coordinate::new(c[2], c[3])),
        None => panic!("Regex failed!"),
    }
}

fn part_1(input: &str, row: isize) -> usize {
    // parse input
    let pairs = input.lines().map(parse).collect::<Vec<_>>();

    let mut cannot = HashSet::<Coordinate>::new();
    for (sensor, beacon) in pairs.iter() {
        let distance = sensor.manhattan_distance(&beacon) as isize;

        // check if we need to evaluate the pair
        if row < sensor.y - distance || row > sensor.y + distance {
            continue;
        }

        // create coordinates to check
        for x in (sensor.x - distance)..=(sensor.x + distance) {
            let tmp = Coordinate::new(x, row);
            // skip coordinate if not within manhattan distance of sensor
            if tmp.manhattan_distance(&sensor) as isize > distance {
                continue;
            }

            // skip if there is a beacon or sensor
            if pairs
                .iter()
                .any(|(sensor, beacon)| tmp == *sensor || tmp == *beacon)
            {
                continue;
            }
            cannot.insert(tmp);
        }
    }

    cannot.len()
}

fn parse_distance(s: &str) -> (Coordinate, usize) {
    // construct regex pattern to match input line
    let input_regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    // collect matches in vector and convert them to isize
    let captures = input_regex.captures(s).map(|captures| {
        captures
            .iter()
            .skip(1) // skip full match
            .map(|c| c.unwrap().as_str().parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    });

    // return sensor beacon Coordinate pair
    match captures {
        Some(c) => {
            let sensor = Coordinate::new(c[0], c[1]);
            let distance = sensor.manhattan_distance(&Coordinate::new(c[2], c[3]));
            (sensor, distance)
        }
        None => panic!("Regex failed!"),
    }
}

fn part_2(input: &str, max: isize) -> ControlFlow<usize> {
    // parse input
    let pairs = input.lines().map(parse_distance).collect::<Vec<_>>();

    (0..=max).into_par_iter().try_for_each(|x| {
        ControlFlow::Continue('y: for y in 0..=max {
            let tmp = Coordinate::new(x, y);

            for (sensor, distance) in pairs.iter() {
                if tmp.manhattan_distance(&sensor) <= *distance {
                    continue 'y;
                }
            }

            // not within manhattan distance of any sensor
            return ControlFlow::Break(tmp.x as usize * 4000000 + tmp.y as usize);
        })
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {}", part_1(&inp.to_string(), 2000000));
    println!("part 2: {:?}", part_2(&inp.to_string(), 4000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    #[test]
    fn day_15_part_1() {
        assert_eq!(
            super::part_1(
                r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
                10
            ),
            26
        );
    }

    #[test]
    fn day_15_part_2() {
        assert_eq!(
            super::part_2(
                r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
                20
            ),
            ControlFlow::Break(56000011)
        );
    }
}
