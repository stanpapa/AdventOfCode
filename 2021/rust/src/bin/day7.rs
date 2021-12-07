use std::io::Error;

use aoc_2021_rust::utils::input;

fn fuel_cost(positions: &Vec<usize>, destination: &usize, part: bool) -> usize {
    let mut fuel = 0;

    for pos in positions {
        let steps = (*pos as i32 - *destination as i32).abs();

        fuel += if part {
            // part 1:
            // fuel cost is equal to number of steps needed to move to `location`
            steps
        } else {
            // part 2:
            // fuel cost is equal to \sum_{n=1}^{k}, with k being
            // the number of steps needed to move to `location`
            (1..=steps).fold(0, |acc, val| acc + val)
        }
    }

    fuel as usize
}

fn minimal_fuel(positions: &Vec<usize>, part: bool) -> usize {
    let pos_min = positions.iter().min().unwrap();
    let pos_max = positions.iter().max().unwrap();

    let mut min_fuel = fuel_cost(positions, pos_max, part);

    for pos in *pos_min..*pos_max {
        let fuel = fuel_cost(positions, &pos, part);

        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
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
