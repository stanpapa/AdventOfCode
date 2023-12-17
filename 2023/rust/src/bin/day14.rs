use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use libaoc::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Platform = Grid<char>;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn total_load(platform: &Platform) -> usize {
    platform
        .iter()
        .filter(|(_coord, &value)| value == 'O')
        .map(|(coord, _rock)| platform.length - coord.y as usize)
        .sum()
}

fn parse(input: &str) -> Platform {
    // initialise full grid
    let mut platform = Platform::new(input);

    // remove all the '.' from the platform
    let empty = platform
        .iter_mut()
        .filter(|(_, &mut value)| value == '.')
        .map(|(coord, _)| *coord)
        .collect::<HashSet<Coordinate>>();
    empty.iter().for_each(|i| platform.remove(i));

    platform
}

fn tilt(platform: &mut Platform, direction: &Direction) {
    // find round_rocks
    let mut rocks_round = platform
        .iter()
        .filter(|(_coord, &ch)| ch == 'O')
        .map(|(coord, _ch)| *coord)
        .collect::<Vec<Coordinate>>();
    rocks_round.sort_by(|a, b| match direction {
        Direction::North => a.y.cmp(&b.y),
        Direction::West => a.x.cmp(&b.x),
        Direction::South => b.y.cmp(&a.y),
        Direction::East => b.x.cmp(&a.x),
    });

    // iterate over all round rocks and move them until they hit a barrier
    rocks_round.iter().for_each(|rock| {
        let mut rock_new = Coordinate::new(rock.x, rock.y);
        loop {
            match direction {
                Direction::North => {
                    if rock_new.y == 0 {
                        break;
                    }

                    rock_new.y -= 1;

                    if platform.get(&rock_new).is_some() {
                        rock_new.y += 1;
                        break;
                    }
                }
                Direction::West => {
                    if rock_new.x == 0 {
                        break;
                    }

                    rock_new.x -= 1;

                    if platform.get(&rock_new).is_some() {
                        rock_new.x += 1;
                        break;
                    }
                }
                Direction::South => {
                    if rock_new.y as usize == platform.length - 1 {
                        break;
                    }

                    rock_new.y += 1;

                    if platform.get(&rock_new).is_some() {
                        rock_new.y -= 1;
                        break;
                    }
                }
                Direction::East => {
                    if rock_new.x as usize == platform.width - 1 {
                        break;
                    }

                    rock_new.x += 1;

                    if platform.get(&rock_new).is_some() {
                        rock_new.x -= 1;
                        break;
                    }
                }
            };
        }

        // update platform
        platform.remove(rock);
        platform.insert(rock_new, 'O');
    })
}

fn part_1(input: &str) -> usize {
    let mut platform = parse(input);
    tilt(&mut platform, &Direction::North);
    total_load(&platform)
}

fn part_2(input: &str) -> usize {
    // input
    let mut platform = parse(input);
    let ncycles = 1_000_000_000;

    // map for cycle detection
    let mut seen = HashMap::<String, usize>::new();

    // closure for performing a full cycle
    let cycle = |platform: &mut Platform| {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            tilt(platform, &direction);
        }
    };

    // iterate over cycles
    'outer: for i in 0..ncycles {
        // add current platform to map
        seen.insert(platform.to_string(), i);

        // cycle platform
        cycle(&mut platform);

        // if we found the same platform, we have found a cycle
        // cycle length is calculated and we fast forward to the end
        if seen.contains_key(platform.to_string().as_str()) {
            let cycle_len = i + 1 - seen[platform.to_string().as_str()];
            let remaining = ncycles - i - 1;
            let remaining = remaining % cycle_len;

            // remaining is the number of steps we need to take to
            // from where we are at to get to the same position that
            // 1_000_000_000 steps would have taken us.
            for _ in 0..remaining {
                cycle(&mut platform);
            }

            // break, so we can calculate the load
            break 'outer;
        }
    }

    // calculate total load after final cycle
    total_load(&platform)
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn total_load() {
        assert_eq!(
            super::total_load(&super::Platform::new(
                r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            )),
            136
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 136);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 64);
    }
}
