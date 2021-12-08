use itertools::Itertools;
use std::{io::Error, str::FromStr};

use aoc_2021_rust::utils::input;
//                     0       1     2       3       4      5       6      7       8        9
static DIGITS: [&str; 10] = ["abcdeg","ab","acdfg","abcdf","abef","bcdef","bcdefg","abd","abcdefg","abcdef"];
// static DIGITS: [&str; 10] = [
//     "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdfg", "acf", "abcdefg", "abcdfg",
// ];

struct Note {
    signal_patterns: [String; 10],
    output: [String; 4],
}

impl FromStr for Note {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split string in 10 signal patterns and 4 digit output
        let split: Vec<String> = s.split(" | ").map(String::from).collect();
        let signal_patterns = split[0]
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
            .try_into()
            .unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", 10, v.len())
            });
        let output = split[1]
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
            .try_into()
            .unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", 4, v.len())
            });

        Ok(Self {
            signal_patterns,
            output,
        })
    }
}

fn part_1(notes: &Vec<Note>) -> usize {
    notes
        .iter()
        .map(|note| {
            note.output
                .iter()
                .filter(|o| [2, 3, 4, 7].contains(&o.len()))
                .count()
        })
        .sum()
}

fn convert_to_digit(perm: &[char], s: &String) -> Option<usize> {
    // apply permutation to signal
    let digit = s
        .chars()
        .map(|c| perm[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();

    // return index(=number) if the converted digit matches an actual digit
    DIGITS.iter().position(|&s| s == digit)
}

// check if the current permutation produces a valid 4-number digit
fn check_permutation(perm: &[char], note: &Note) -> Option<usize> {
    // check if permutation is valid by checking if it converts to a proper digit
    let invalid = note
        .signal_patterns
        .iter()
        .map(|signal| convert_to_digit(perm, signal))
        .any(|o| o.is_none());

    if invalid {
        return None;
    }

    // if permutation is valid translate all output signals and sum them up
    let sum_digits = note
        .output
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| convert_to_digit(perm, digit).unwrap() * 10usize.pow(i as u32))
        .sum();

    Some(sum_digits)
}

// check all possible permutations of signals until the right one has been found
// sum the resulting values
fn part_2(notes: &Vec<Note>) -> usize {
    notes
        .iter()
        .map(|note| {
            "abcdefg"
                .chars()
                .permutations(7)
                .find_map(|permutation| check_permutation(&permutation, note))
                .unwrap()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
