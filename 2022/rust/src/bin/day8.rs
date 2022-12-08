use std::error::Error;

use aoc_lib::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Forest = Grid<u8>;

const DIRECTIONS: [Coordinate; 4] = [
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 0, y: -1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: -1, y: 0 },
];

fn part_1(forest: &Forest) -> usize {
    let mut count = 0;
    for (coord, weight) in forest.iter_inner() {
        'outer: for direction in DIRECTIONS {
            let mut step = 1;
            loop {
                let coord_next =
                    Coordinate::new(coord.x + direction.x * step, coord.y + direction.y * step);
                if forest[coord_next] >= *weight {
                    break;
                }
                if forest.is_edge(&coord_next) {
                    count += 1;
                    break 'outer;
                }
                step += 1;
            }
        }
    }

    count + forest.len_edge()
}

fn scenic_score(forest: &Forest, tree: (&Coordinate, &u8)) -> usize {
    let coord = tree.0;
    let weight = tree.1;
    let mut scenic_score = 1;

    for direction in DIRECTIONS {
        let mut step = 1;
        loop {
            let coord_next =
                Coordinate::new(coord.x + direction.x * step, coord.y + direction.y * step);

            match forest.get(&coord_next) {
                Some(w) => {
                    if weight <= w || forest.is_edge(&coord_next) {
                        scenic_score *= step;
                        break;
                    }
                }
                None => break,
            }

            step += 1;
        }
    }

    scenic_score as usize
}

fn part_2(forest: &Forest) -> usize {
    *forest
        .iter()
        .map(|tree| scenic_score(forest, tree))
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
}

fn solve(input: &str, part1: bool) -> usize {
    let forest = Forest::new(input);
    match part1 {
        true => part_1(&forest),
        false => part_2(&forest),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {:?}", solve(&inp.to_string(), true));
    println!("part 2: {:?}", solve(&inp.to_string(), false));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_8_part_1() {
        assert_eq!(
            super::solve(
                r#"30373
25512
65332
33549
35390"#,
                true
            ),
            21
        );
    }

    #[test]
    fn day_8_part_2() {
        assert_eq!(
            super::solve(
                r#"30373
25512
65332
33549
35390"#,
                false
            ),
            8
        );
    }
}
