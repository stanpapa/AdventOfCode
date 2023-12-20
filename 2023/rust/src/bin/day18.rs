use std::io::Error;

use libaoc::{grid::coordinate::Coordinate, io::input::Input, math::polygon::interior_points};

fn dig(trench: &mut Vec<Coordinate>, direction: char, distance: usize) {
    let mut current = trench.last().unwrap().clone();
    for i in 0..distance {
        match direction {
            'U' => current += Coordinate::new(0, -1),
            'D' => current += Coordinate::new(0, 1),
            'L' => current += Coordinate::new(-1, 0),
            'R' => current += Coordinate::new(1, 0),
            _ => panic!("Invalid direction"),
        }
        trench.push(current);
    }
}

fn dig_trench(input: &str, part_1: bool) -> Vec<Coordinate> {
    let mut trench = vec![Coordinate::new(0, 0)];

    // iterate over all instructions to dig the trench
    input.lines().for_each(|line| {
        let split = line.split(' ').collect::<Vec<&str>>();
        match part_1 {
            true => dig(
                &mut trench,
                split[0].parse::<char>().unwrap(),
                split[1].parse::<usize>().unwrap(),
            ),
            false => {
                // convert instruction
                let distance = usize::from_str_radix(&split[2][2..=6], 16).unwrap();
                let direction = match &split[2][7..=7] {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    "3" => 'U',
                    _ => panic!("Invalid direction"),
                };
                dig(&mut trench, direction, distance);
            }
        }
    });

    trench
}

fn solve(input: &str, part_1: bool) -> usize {
    // dig trench. store the trench as a vector
    let trench = dig_trench(input, part_1);

    // calculate how many cubic meters the lagoon can contain
    // by computing the number of interior points of the trench
    // combined with the length of the trench itself
    // correct with -1, because the starting point is counted twice
    interior_points(&trench) + trench.len() - 1
}

fn main() -> Result<(), Error> {
    let input = Input::new().to_string();

    println!("{}", solve(&input, true));
    println!("{}", solve(&input, false));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_1() {
        assert_eq!(super::solve(INPUT, true), 62);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve(INPUT, false), 952408144115);
    }
}
