use std::collections::HashSet;
use std::io::Error;
use std::str::FromStr;

use aoc_lib::io::input::Input;

struct Rucksack {
    compartment_1: String,
    compartment_2: String,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let half = s.len() / 2;
        Ok(Rucksack {
            compartment_1: String::from(&s[0..half]),
            compartment_2: String::from(&s[half..]),
        })
    }
}

impl Rucksack {
    fn priority(&self) -> u32 {
        let unique_1: HashSet<char> = HashSet::from_iter(self.compartment_1.chars());
        let unique_2: HashSet<char> = HashSet::from_iter(self.compartment_2.chars());

        unique_2
            .iter()
            .filter(|c| unique_1.contains(c))
            .map(priority)
            .sum()
    }
}

fn priority(item: &char) -> u32 {
    // a-z -> 1-26
    if (*item as u32) >= 97 && (*item as u32) <= 122 {
        return *item as u32 - 96;
    }
    // A-Z -> 27-52
    *item as u32 - 38
}

fn part_1(rucksacks: &str) -> u32 {
    rucksacks
        .lines()
        .map(|l| {
            Rucksack::from_str(l)
                .expect("Unable to parse Rucksack")
                .priority()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new();

    println!("part 1: {}", part_1(&inp.to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_3_part_1() {
        assert_eq!(0, 0);
    }

    #[test]
    fn day_3_part_2() {
        assert_eq!(0, 0);
    }
}
