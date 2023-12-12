use std::io::Error;

use libaoc::io::input::Input;

use itertools::Itertools;
use rayon::prelude::*;

fn possible_arrangements(condition_record: &str, broken: &[u8]) -> usize {
    let damaged_known = condition_record.chars().filter(|ch| *ch == '#').count();
    let damaged_total = broken.iter().map(|i| *i as usize).sum::<usize>();
    let unknown = condition_record
        .char_indices()
        .filter(|(_i, ch)| *ch == '?')
        .map(|(i, _ch)| i)
        .collect::<Vec<usize>>();

    // generate all combinations and count only valid ones
    unknown
        .iter()
        .combinations(damaged_total - damaged_known)
        .filter(|combination| {
            let mut tmp = condition_record.to_string();
            for &i in combination {
                tmp.replace_range(i..=i, "#");
            }
            tmp = tmp.replace('?', ".");
            let check = tmp
                .split('.')
                .filter(|s| !s.is_empty())
                .map(|s| s.len() as u8)
                .collect::<Vec<u8>>();
            check.iter().zip(broken.iter()).all(|(a, b)| a == b)
        })
        .count()
}

/// Parse the input. Duplicate the record `unfold` times
fn parse(input: &str, unfold: usize) -> Vec<(String, Vec<u8>)> {
    input
        .lines()
        .map(|l| {
            let (condition_record, numbers) = l.split_once(' ').unwrap();
            (
                std::iter::repeat(condition_record)
                    .take(unfold)
                    .collect::<Vec<&str>>()
                    .join("?"),
                std::iter::repeat(numbers)
                    .take(unfold)
                    .collect::<Vec<&str>>()
                    .join(",")
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn solve(input: &str, unfold: usize) -> usize {
    // parse input and calculate the sum of all valid combinations in the records
    parse(input, unfold)
        .par_iter()
        .map(|(condition_record, broken)| possible_arrangements(condition_record, broken))
        .sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", solve(&inp, 1));
    // println!("{}", solve(&inp, 5));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    const INPUT: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1() {
        assert_eq!(solve(INPUT, 1), 21);
    }

    // #[test]
    // fn part_2() {
    //     assert_eq!(solve(INPUT, 5), 525152);
    // }
}
