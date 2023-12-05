use std::{collections::HashSet, io::Error};

use libaoc::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Schematic = Grid<char>;

const DIRECTIONS: [Coordinate; 8] = [
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 0, y: -1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: -1, y: 0 },
    Coordinate { x: 1, y: 1 },
    Coordinate { x: 1, y: -1 },
    Coordinate { x: -1, y: 1 },
    Coordinate { x: -1, y: -1 },
];

fn is_special_character(ch: &char) -> bool {
    !(ch.is_alphanumeric() || *ch == '.')
}

fn find_surrounding_numbers(
    schematic: &Schematic,
    coord: &Coordinate,
    numbers: &mut Vec<u32>,
    seen: &mut HashSet<Coordinate>,
) {
    for direction in DIRECTIONS {
        // check if neighbour can be found in Schematic
        let neighbour = *coord + direction;
        if let Some(c) = schematic.get(&neighbour) {
            // ignore neighbour if it is not a number or if we already processsed this number
            if !c.is_numeric() || seen.contains(&neighbour) {
                continue;
            }
            let mut number = c.to_digit(10).unwrap();
            seen.insert(neighbour);

            // sweep left for beginning of number
            let mut left = neighbour;
            loop {
                left = left + Coordinate::new(-1, 0);
                if let Some(l) = schematic.get(&left) {
                    seen.insert(left);
                    if !l.is_numeric() {
                        break;
                    }
                    number = number
                        + l.to_digit(10).unwrap() * 10_u32.pow(neighbour.x as u32 - left.x as u32);
                } else {
                    break;
                }
            }

            // sweep right for end of number
            let mut right = neighbour;
            loop {
                right = right + Coordinate::new(1, 0);
                if let Some(r) = schematic.get(&right) {
                    seen.insert(right);
                    if !r.is_numeric() {
                        break;
                    }
                    number = number * 10 + r.to_digit(10).unwrap();
                } else {
                    break;
                }
            }

            numbers.push(number);
        }
    }
}

fn part_1(schematic: &Schematic) -> u32 {
    let mut seen = HashSet::<Coordinate>::new();
    let mut numbers = Vec::new();

    schematic
        .iter()
        .filter(|(_, ch)| is_special_character(ch))
        .for_each(|(coord, _ch)| {
            find_surrounding_numbers(schematic, coord, &mut numbers, &mut seen);
        });

    numbers.iter().sum()
}

fn part_2(schematic: &Schematic) -> u32 {
    schematic
        .iter()
        .filter(|(_, ch)| **ch == '*')
        .fold(0, |acc, (coord, _ch)| {
            let mut seen = HashSet::<Coordinate>::new();
            let mut numbers = Vec::new();
            find_surrounding_numbers(schematic, coord, &mut numbers, &mut seen);

            // a '*' is only a gear when it is surrounder by 2 numbers
            if numbers.len() == 2 {
                acc + numbers.iter().product::<u32>()
            } else {
                acc
            }
        })
}

fn main() -> Result<(), Error> {
    let inp = Input::new();
    let schematic = Schematic::new(&inp.to_string());

    println!("{}", part_1(&schematic));
    println!("{}", part_2(&schematic));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            super::part_1(&Schematic::new(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            4361
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            super::part_2(&Schematic::new(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            467835
        );
    }
}
