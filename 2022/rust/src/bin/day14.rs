use std::{
    cmp::{max, min},
    error::Error,
};

use aoc_lib::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Cave = Grid<char>;

fn pour(cave: &Cave) -> Option<Coordinate> {
    let mut grain = Coordinate::new(500, 0);
    let down = Coordinate::new(0, 1);
    let left = Coordinate::new(-1, 1);
    let right = Coordinate::new(1, 1);

    let bottom = *cave
        .iter()
        .map(|(key, _)| key.y)
        .collect::<Vec<_>>()
        .iter()
        .max()
        .unwrap();

    loop {
        // println!("{}", grain);

        if grain.y == bottom {
            return None;
        }

        match cave.get(&(grain + down)) {
            Some(_) => (),
            None => {
                grain += down;
                continue;
            }
        }
        match cave.get(&(grain + left)) {
            Some(_) => (),
            None => {
                grain += left;
                continue;
            }
        }
        match cave.get(&(grain + right)) {
            Some(_) => (),
            None => {
                grain += right;
                continue;
            }
        }

        break;
    }

    Some(grain)
}

fn solve(input: &str) -> usize {
    // place rocks on map
    let mut cave = Cave::default();
    // let mut cave = Cave::new_with_value();
    for line in input.lines() {
        let coords = line
            .split(" -> ")
            .map(|x| x.parse::<Coordinate>().unwrap())
            .collect::<Vec<Coordinate>>();
        // println!("{:?}", coords);
        coords.windows(2).for_each(|c| {
            // println!("{} -> {}", c[0], c[1]);
            match c[0].x == c[1].x {
                true => {
                    for y in min(c[0].y, c[1].y)..=max(c[0].y, c[1].y) {
                        cave.insert(Coordinate::new(c[0].x, y), '#');
                    }
                }
                false => {
                    for x in min(c[0].x, c[1].x)..=max(c[0].x, c[1].x) {
                        cave.insert(Coordinate::new(x, c[0].y), '#');
                    }
                }
            };
        });
        // println!();
    }

    let mut grains = 0;
    loop {
        if let Some(grain) = pour(&cave) {
            cave.insert(grain, 'o');
            grains += 1;

            // visualize in terminal
            // let mut clear = std::process::Command::new("clear");
            // clear.status().expect("failed to execute command");
            // println!("{}", cave);
            // std::thread::sleep(std::time::Duration::from_millis(50));
        } else {
            break;
        }
    }

    grains
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {}", solve(&inp.to_string()));
    //     println!(
    //         "part 1: {}",
    //         solve(
    //             &r#"498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9"#
    //         )
    //     );
    // println!("part 2:\n{}", part_2(&inp.to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_14_part_1() {
        assert_eq!(
            super::solve(
                r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#
            ),
            24
        );
    }

    #[test]
    fn day_14_part_2() {
        assert_eq!(
            super::solve(
                r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#
            ),
            93
        );
    }
}
