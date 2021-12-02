use std::io::Error;

use aoc_2021_rust::utils::input;

// ------------------------------------------------------------
// Part 1
// You can use window to iterate over a range/window of items
// filter can be used instead of an if
// we then count the number of occurrences the filter lets through
// ------------------------------------------------------------
fn part_1(sweeps: &Vec<i32>) -> Result<u32, Error> {
    Ok(sweeps.windows(2).filter(|w| w[0] < w[1]).count() as u32)
}

// ------------------------------------------------------------
// Part 2
// We use the same principle as for part 1, with the difference
// being the window size and filter condition.
// the exercise is to check [a,b,c].sum() - [b,c,d].sum() = a - d
// So, it suffices to check in windows of 4 and only check a - d
// ------------------------------------------------------------
fn part_2(sweeps: &Vec<i32>) -> Result<u32, Error> {
    Ok(sweeps.windows(4).filter(|w| w[0] < w[3]).count() as u32)
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(&input)?);

    Ok(())
}
