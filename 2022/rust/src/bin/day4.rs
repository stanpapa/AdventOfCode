use std::{error::Error, str::FromStr};

use aoc_lib::io::input::Input;

#[derive(Debug)]
struct Task(u64, u64);

impl FromStr for Task {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(pair) = s.split_once('-') {
            return Ok(Task(
                pair.0.parse::<u64>().unwrap(),
                pair.1.parse::<u64>().unwrap(),
            ));
        }
        panic!("Malformatted task")
    }
}

fn full_overlap(assignments: &str) -> bool {
    // obtaion individual assignments
    let assignment = assignments.split_once(',').unwrap();

    // convert assignments to numerical values
    let a1 = Task::from_str(assignment.0).unwrap();
    let a2 = Task::from_str(assignment.1).unwrap();

    (a1.0 >= a2.0 && a1.1 <= a2.1) || (a2.0 >= a1.0 && a2.1 <= a1.1)
}

fn part_1(input: &str) -> usize {
    input.lines().filter(|l| full_overlap(l)).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {}", part_1(&inp.to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_4_part_1() {
        assert_eq!(
            super::part_1(
                r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            ),
            2
        );
    }

    #[test]
    fn day_4_part_2() {
        assert_eq!(0, 1);
    }
}
