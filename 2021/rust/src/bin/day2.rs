use std::io::Error;

use aoc_2021_rust::utils::input;

struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn change_part_1(&mut self, command: &str) {
        let split: Vec<&str> = command.split_whitespace().collect();
        let c = split[0];
        let step = split[1].parse::<u32>().expect("not an int");
        match c {
            "forward" => self.horizontal += step,
            "down" => self.depth += step,
            "up" => self.depth -= step,
            _ => (),
        }
    }

    fn change_part_2(&mut self, command: &str) {
        let split: Vec<&str> = command.split_whitespace().collect();
        let c = split[0];
        let step = split[1].parse::<u32>().expect("not an int");
        match c {
            "forward" => {
                self.horizontal += step;
                self.depth += self.aim * step;
            }
            "down" => self.aim += step,
            "up" => self.aim -= step,
            _ => (),
        }
    }
}

fn part_1(commands: &Vec<String>) -> u32 {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands.iter() {
        pos.change_part_1(&command);
    }

    pos.horizontal * pos.depth
}

fn part_2(commands: &Vec<String>) -> u32 {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands.iter() {
        pos.change_part_2(&command);
    }

    pos.horizontal * pos.depth
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}