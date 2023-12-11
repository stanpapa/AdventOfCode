use std::{cmp::Ordering, collections::HashSet, io::Error};

use libaoc::{
    grid::{coordinate::Coordinate, grid::Grid, manhattan_distance::ManhattanDistance},
    io::input::Input,
};

type Image = Grid<char>;

fn solve(input: &str, expansion_rate: usize) -> usize {
    let image = Image::new(input);

    // collect locations of galaxies
    let galaxies = image
        .iter()
        .filter(|(_coord, &ch)| ch == '#')
        .map(|(coord, _ch)| *coord)
        .collect::<Vec<Coordinate>>();

    // find empty rows/columns for expansion
    let empty_rows = (0..image.width as isize)
        .filter(|x| !galaxies.iter().any(|g| g.x == *x))
        .collect::<HashSet<isize>>();

    let empty_cols = (0..image.length as isize)
        .filter(|y| !galaxies.iter().any(|g| g.y == *y))
        .collect::<HashSet<isize>>();

    // calculate shortest (Manhattan distance) between all unique pairs of galaxies
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            // quick access
            let gi = galaxies[i];
            let gj = galaxies[j];

            // calculate horizontal expansion
            let expansion_horizontal = match gi.x.cmp(&gj.x) {
                Ordering::Greater => (gj.x..gi.x).filter(|x| empty_rows.contains(x)).count(),
                Ordering::Less => (gi.x..gj.x).filter(|x| empty_rows.contains(x)).count(),
                Ordering::Equal => 0,
            };

            // calculate vertical expansion
            let expansion_vertical = match gi.y.cmp(&gj.y) {
                Ordering::Greater => (gj.y..gi.y).filter(|y| empty_cols.contains(y)).count(),
                Ordering::Less => (gi.y..gj.y).filter(|y| empty_cols.contains(y)).count(),
                Ordering::Equal => 0,
            };

            // append shortest distance between galaxies to sum total
            sum += gi.manhattan_distance(&gj)
                + (expansion_rate - 1) * (expansion_vertical + expansion_horizontal);
        }
    }

    sum
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", solve(&inp, 2));
    println!("{}", solve(&inp, 1000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                2
            ),
            374
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                10
            ),
            1030
        );

        assert_eq!(
            solve(
                r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                100
            ),
            8410
        );
    }
}
