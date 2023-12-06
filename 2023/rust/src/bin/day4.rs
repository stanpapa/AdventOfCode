use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use libaoc::io::input::Input;

fn matches(card: &str) -> u32 {
    let split = card.split_once(" | ").unwrap();

    // collect numbers on left and right side of '|' in HashSets
    let numbers_left = split.0.split_whitespace().collect::<HashSet<&str>>();
    let numbers_right = split.1.split_whitespace().collect::<HashSet<&str>>();

    // return number of matches between left and right set of numbers
    numbers_left.intersection(&numbers_right).count() as u32
}

fn part_1(scratchcards: &str) -> u32 {
    // loop over cards and accumulate score
    scratchcards.lines().fold(0, |acc, scratchcard| {
        // calculate number of matches
        let n = matches(scratchcard.split_once(": ").unwrap().1);

        // if there is at least one match, then calculate the score
        if n >= 1 {
            return acc + (1 << (n - 1));
        }

        // no matches means the score stays the same
        acc
    })
}

fn part_2(scratchcards: &str) -> u32 {
    // keep track of how many card copies there are
    let mut cards = (0..scratchcards.lines().count())
        .map(|i| (i, 1))
        .collect::<HashMap<usize, u32>>();

    // loop over all original cards
    for (i, scratchcard) in scratchcards.lines().enumerate() {
        // calculate number of matches
        let n = matches(scratchcard) as usize;

        // if there are no matches, then we don't produce new copies
        if n == 0 {
            continue;
        }

        // number of current card copies
        let copies = *cards.get(&i).unwrap();

        // add current number of copies to copies of card i+1 until i+1+n
        (i + 1..i + 1 + n).for_each(|j| {
            if let Some(val) = cards.get_mut(&j) {
                *val += copies;
            }
        });
    }

    // compute total number of scratch cards
    cards.values().sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 13);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 30);
    }
}
