use std::io::Error;

use aoc_2021_rust::utils::input;


fn procreate(mut fish: [usize; 9], days: usize) -> usize {
    for _day in 0..days {
        // rotate left: move the first n elements to the back of the vector
        // element n becomes the first element
        // this simulates the spawning of new fish,
        // since we spawn as many fish on day 8 as there were fish on day 0
        fish.rotate_left(1);
        // the old fish reset to 6 days instead of 8
        fish[6] += fish[8];
    }
    fish.iter().sum()
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_string()?;
    let mut fish = [0; 9];

    // we gather an array with the lanternfish grouped by how many days they
    // have left to spawn a new fish (from 0-8)
    // todo: can I directly use the mapped value, instead of collecting and iterating?
    input
        .split(',')
        .map(|f| f.parse::<usize>().expect("Not a number"))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|f| fish[f] += 1);

    println!("Part 1: {}", procreate(fish, 80));
    println!("Part 1: {}", procreate(fish, 256));

    Ok(())
}
