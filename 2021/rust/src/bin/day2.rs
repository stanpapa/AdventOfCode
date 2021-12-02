use std::{io::Error, str::FromStr};

use aoc_2021_rust::utils::input;

// Basically copied from repo below
// it shows some nice features, such as implementing a trait for a custom struct/enum
// https://gitlab.com/xelivous/aoc2021/-/blob/master/src/day2.rs

struct Command {
    direction: Direction,
    size: u32,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<_> = s.split_whitespace().collect();
        let direction = data[0].parse()?;
        let size = data[1].parse().map_err(|_| "can't parse size")?;

        Ok(Self { direction, size })
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err("Not a valid direction"),
        }
    }
}

struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

fn part_1(commands: &Vec<Command>) -> u32 {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command.direction {
            Direction::Forward => pos.horizontal += command.size,
            Direction::Down => pos.depth += command.size,
            Direction::Up => pos.depth -= command.size,
        }
    }

    pos.horizontal * pos.depth
}

fn part_2(commands: &Vec<Command>) -> u32 {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command.direction {
            Direction::Forward => {
                pos.horizontal += command.size;
                pos.depth += command.size * pos.aim;
            }
            Direction::Down => pos.aim += command.size,
            Direction::Up => pos.aim -= command.size,
        }
    }

    pos.horizontal * pos.depth
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
