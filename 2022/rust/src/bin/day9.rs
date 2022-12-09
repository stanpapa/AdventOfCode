use std::{collections::HashSet, error::Error, str::FromStr};

use aoc_lib::{
    grid::{chebyshev_distance::ChebyshevDistance, coordinate::Coordinate},
    io::input::Input,
};

#[derive(Debug)]
struct Motion {
    direction: Coordinate,
    step: usize,
}

impl FromStr for Motion {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp = s
            .split_once(' ')
            .unwrap_or_else(|| panic!("Invalid motion"));

        Ok(Self {
            direction: match tmp.0 {
                "R" => Coordinate::new(1, 0),
                "L" => Coordinate::new(-1, 0),
                "U" => Coordinate::new(0, 1),
                "D" => Coordinate::new(0, -1),
                _ => panic!("Invalid direction: {}", tmp.0),
            },
            step: tmp
                .1
                .parse()
                .unwrap_or_else(|_| panic!("Invalid step size: {}", tmp.1)),
        })
    }
}

fn follow(head: &Coordinate, tail: &Coordinate) -> Coordinate {
    if head.chebyshev_distance(tail) <= 1 {
        return *tail;
    }

    let diff = *head - *tail;
    Coordinate::new(tail.x + diff.x.signum(), tail.y + diff.y.signum())
}

fn solve(input: &str, part1: bool) -> usize {
    let length = match part1 {
        true => 2,
        false => 10,
    };

    let motions = input
        .lines()
        .map(|l| l.parse::<Motion>().unwrap())
        .collect::<Vec<_>>();

    let mut rope = vec![Coordinate::new(0, 0); length];
    let mut visited = HashSet::new();

    motions.iter().for_each(|m| {
        for _ in 1..=m.step {
            rope[0] += m.direction; // head
            for i in 1..rope.len() {
                rope[i] = follow(&rope[i - 1], &rope[i]);
            }
            visited.insert(rope[rope.len() - 1]);
        }
    });

    visited.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {:?}", solve(&inp.to_string(), true));
    println!("part 2: {:?}", solve(&inp.to_string(), false));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_9_part_1() {
        assert_eq!(
            super::solve(
                r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
                true
            ),
            13
        );
    }

    #[test]
    fn day_9_part_2() {
        assert_eq!(
            super::solve(
                r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
                false
            ),
            1
        );

        assert_eq!(
            super::solve(
                r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
                false
            ),
            36
        );
    }
}
