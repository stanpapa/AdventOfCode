use std::{collections::HashMap, io::Error};

use libaoc::io::input::Input;

fn solve(lists: &str) -> i32 {
    // parse list of numbers on left and right
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    lists.lines().for_each(|l| {
        let (a, b) = l.split_once("   ").unwrap();
        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    });

    // sort lists
    left.sort();
    right.sort();

    // calculate total distance between lists
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn solve_2(lists: &str) -> u32 {
    // parse list of numbers and count occurences
    let mut left = HashMap::<u32, u32>::new();
    let mut right = HashMap::<u32, u32>::new();
    lists.lines().for_each(|l| {
        let (a, b) = l.split_once("   ").unwrap();
        left.entry(a.parse().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        right
            .entry(b.parse().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    // calculate similarity score
    left.iter().fold(0, |acc, (key, value)| {
        acc + key * value * right.get(key).unwrap_or(&0)
    })
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", solve(&inp));
    println!("{}", solve_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::solve_2;

    use super::solve;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                r"3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_2(
                r"3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31
        );
    }
}
