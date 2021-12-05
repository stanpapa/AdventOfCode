use std::{collections::HashMap, fmt, io::Error, ops, str::FromStr};

use aoc_2021_rust::utils::input;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // todo: use tuple to destruct
        let coordinate: Vec<i32> = s
            .split(',')
            .map(|n| n.parse().expect("Not a number"))
            .collect();

        Ok(Coordinate {
            x: coordinate[0],
            y: coordinate[1],
        })
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        // println!("{}, {}", self, rhs);
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}


#[derive(Clone, Copy)]
struct Line {
    begin: Coordinate,
    end: Coordinate,
    slope: Coordinate,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.begin, self.end)
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<Coordinate> = s
            .split(" -> ")
            .map(|c| c.parse().expect("Not a coordinate"))
            .collect();

        let begin = coordinates[0];
        let end = coordinates[1];

        let x = (end.x - begin.x).signum();
        let y = (end.y - begin.y).signum();
        let slope = Coordinate { x, y };

        Ok(Line { begin, end, slope })
    }
}

// todo: Implement custom iterator
// if I use this then the iteration doesn't start at the beginning,
// but at beginning + slope...
// how to solve? into_iter?/iter
impl Iterator for Line {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        // todo!
        if self.begin == self.end {
            None
        } else {
            self.begin += self.slope;
            Some(self.begin)
        }

    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.begin.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.begin.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }
}

fn count_intersect(lines: &Vec<Line>) -> usize {
    let mut map = HashMap::new();

    for line in lines {
        let mut point = line.begin;
        let end = line.end + line.slope;

        while point != end {
            *map.entry(point).or_insert(0) += 1;
            point += line.slope;
        }
    }

    map.values().filter(|&&val| val > 1).count()
}

fn part_1(lines: &Vec<Line>) -> usize {
    // only consider horizontal and vertical lines
    let non_diagonals: Vec<_> = lines
        .to_vec()
        .into_iter()
        .filter(|l| !l.is_diagonal())
        .collect();

    count_intersect(&non_diagonals)
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", count_intersect(&input));

    Ok(())
}
