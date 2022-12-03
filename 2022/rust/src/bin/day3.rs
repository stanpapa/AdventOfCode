use std::collections::HashSet;
use std::io::Error;
use std::str::FromStr;

use aoc_lib::io::input::Input;

struct Rucksack {
    contents: String,
    half: usize,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            contents: String::from(s),
            half: s.len() / 2,
        })
    }
}

impl Rucksack {
    fn priority(&self) -> u32 {
        let unique_1: HashSet<char> =
            HashSet::from_iter(self.contents.as_str()[0..self.half].chars());
        let unique_2: HashSet<char> =
            HashSet::from_iter(self.contents.as_str()[self.half..].chars());

        unique_1
            .iter()
            .filter(|c| unique_2.contains(c))
            .map(priority)
            .sum()
    }

    fn contains(&self, item: &char) -> bool {
        self.contents.contains(*item)
    }

    fn items_in_common(&self, other: &Rucksack) -> HashSet<char> {
        let mut items = HashSet::new();

        self.contents.chars().for_each(|c| {
            if other.contains(&c) {
                items.insert(c);
            }
        });

        items
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
        .map(|l| Rucksack::from_str(l).unwrap().priority())
        .sum()
}

fn part_2(rucksacks: &str) -> u32 {
    rucksacks
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let set1 = Rucksack::from_str(group[0])
                .unwrap()
                .items_in_common(&Rucksack::from_str(group[1]).unwrap());
            let set2 = Rucksack::from_str(group[0])
                .unwrap()
                .items_in_common(&Rucksack::from_str(group[2]).unwrap());
            let intersection = set1.intersection(&set2);
            priority(intersection.last().unwrap())
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new();

    println!("part 1: {}", part_1(&inp.to_string()));
    println!("part 2: {}", part_2(&inp.to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_3_part_1() {
        assert_eq!(
            super::part_1(
                r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            ),
            157
        );
    }

    #[test]
    fn day_3_part_2() {
        assert_eq!(
            super::part_2(
                r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            ),
            70
        );
    }
}
