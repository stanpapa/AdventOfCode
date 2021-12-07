use std::io::Error;

use aoc_2021_rust::utils::input;

fn gauss(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn minimal_fuel(positions: &Vec<i32>, part: bool) -> i32 {
    // by adding the reference operator, (min,max) are bound to the actual values
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();

    if part {
        // part 1:
        // fuel cost is equal to number of steps needed to move to `destination`
        (min..max)
            .map(|destination| {
                positions
                    .iter()
                    .map(|pos| (destination - pos).abs())
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    } else {
        // part 2:
        // fuel cost is equal to \sum_{n=1}^{k} = 1/2 k(k+1), with k being
        // the number of steps needed to move to `destination`
        (min..max)
            .map(|destination| {
                positions
                    .iter()
                    .map(|pos| gauss((destination - pos).abs()))
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    }
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_string()?;

    let positions = input
        .split(',')
        .map(|num| num.parse().expect("Not a number"))
        .collect();

    println!("Part 1: {}", minimal_fuel(&positions, true));
    println!("Part 2: {}", minimal_fuel(&positions, false));

    Ok(())
}
