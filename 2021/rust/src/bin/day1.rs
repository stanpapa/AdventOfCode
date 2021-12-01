use std::io::Error;

// use crate::utils;
use aoc_2021_rust::utils::input;

pub fn main() -> Result<(), Error> {
    let input = input::read_input_as_ints()?;

    // part 1
    let mut increases = 0;
    let mut last_number = input[0];
    for i in 1..input.len() {
        if input[i] > last_number {
            increases += 1;
        }
        last_number = input[i];
    }

    println!("Part 1: {}", increases);

    // part 2
    increases = 0;
    last_number = input[0] + input[1] + input[2];
    for i in 1..input.len() - 2 {
        let sum = input[i] + input[i + 1] + input[i + 2];
        if sum > last_number {
            increases += 1;
        }
        last_number = sum;
    }

    println!("Part 2: {}", increases);

    Ok(())
}
