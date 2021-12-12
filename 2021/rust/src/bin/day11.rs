use std::io::Error;

use aoc_2021_rust::utils::{coordinate::Coordinate, grid::GridMap, input};

static DIRECTIONS: [Coordinate; 8] = [
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 0, y: -1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: -1, y: 0 },
    Coordinate { x: 1, y: 1 },
    Coordinate { x: 1, y: -1 },
    Coordinate { x: -1, y: 1 },
    Coordinate { x: -1, y: -1 },
];

fn step(octopi: &mut GridMap<u8>) -> usize {
    let mut queue = octopi.iter().map(|(&c, _energy)| c).collect::<Vec<_>>();
    let mut flashes = 0;

    while let Some(coord) = queue.pop() {
        // increase energy by 1 per round or if flashed
        octopi[coord] += 1;

        // only add neighbouring coordinates to the queue if they are flashing
        if octopi[coord] == 10 {
            for direction in DIRECTIONS {
                let neighbour = coord + direction;
                if octopi.contains_key(&neighbour) {
                    queue.push(neighbour);
                }
            }
        }
    }

    for (_c, energy) in octopi.iter_mut() {
        if *energy > 9 {
            *energy = 0;
            flashes += 1;
        }
    }

    flashes
}

// count the number of flashes after 100 steps
fn part_1(octopus_map: &GridMap<u8>, steps: u8) -> usize {
    let mut octopi = octopus_map.clone();
    (0..steps).map(|_| step(&mut octopi)).sum()
}

// find step where all octupi flash simultaneously
// this means that the number of flashes == grid size
fn part_2(octopus_map: &GridMap<u8>) -> usize {
    let mut octopi = octopus_map.clone();
    (0..).find(|_| step(&mut octopi) == octopi.size()).unwrap() + 1
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_string()?;
    let octopus_map = GridMap::<u8>::new(input);

    println!("Part 1: {}", part_1(&octopus_map, 100));
    println!("Part 2: {}", part_2(&octopus_map));

    Ok(())
}
