use std::{
    cmp::{max, min},
    error::Error,
};

use aoc_lib::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Cave = Grid<char>;

fn pour(cave: &Cave, bottom: isize, part_1: bool) -> Option<Coordinate> {
    let mut grain = Coordinate::new(500, 0);
    let moves = [
        Coordinate::new(0, 1),
        Coordinate::new(-1, 1),
        Coordinate::new(1, 1),
    ];

    'outer: loop {
        if part_1 {
            if grain.y == bottom {
                return None;
            }
        } else {
            if grain.y == bottom {
                return Some(grain);
            }

            if cave.get(&Coordinate::new(500, 0)).is_some() {
                return None;
            }
        }

        for m in moves.into_iter() {
            let tmp = grain + m;
            match cave.get(&tmp) {
                Some(_) => (),
                None => {
                    grain = tmp;
                    continue 'outer;
                }
            }
        }

        break;
    }

    Some(grain)
}

fn solve(input: &str, part_1: bool) -> usize {
    // place rocks on map
    let mut cave = Cave::default();
    for line in input.lines() {
        line.split(" -> ")
            .map(|x| x.parse::<Coordinate>().unwrap())
            .collect::<Vec<Coordinate>>()
            .windows(2)
            .for_each(|c| {
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
    }

    let bottom = *cave
        .iter()
        .map(|(key, _)| key.y)
        .collect::<Vec<_>>()
        .iter()
        .max()
        .unwrap()
        + !part_1 as isize;

    let mut grains = 0;
    while let Some(grain) = pour(&cave, bottom, part_1) {
        cave.insert(grain, 'o');
        grains += 1;

        // visualize in terminal
        // let mut clear = std::process::Command::new("clear");
        // clear.status().expect("failed to execute command");
        // println!("{}", cave);
        // std::thread::sleep(std::time::Duration::from_millis(50));
    }

    grains
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {}", solve(&inp.to_string(), true));
    println!("part 2: {}", solve(&inp.to_string(), false));
    //     println!(
    //         "part 1: {}",
    //         solve(
    //             &r#"498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9"#,
    //             true
    //         )
    //     );

    //     println!(
    //         "part 2: {}",
    //         solve(
    //             &r#"498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9"#,
    //             false
    //         )
    //     );

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_14_part_1() {
        assert_eq!(
            super::solve(
                r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
                true
            ),
            24
        );
    }

    #[test]
    fn day_14_part_2() {
        assert_eq!(
            super::solve(
                r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
                false
            ),
            93
        );
    }
}
