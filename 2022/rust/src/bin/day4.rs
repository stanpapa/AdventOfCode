use std::{error::Error, str::FromStr};

use aoc_lib::io::input::Input;

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

fn overlap(assignments: &str, full: bool) -> bool {
    // obtaion individual assignments
    let assignment = assignments.split_once(',').unwrap();

    // convert assignments to numerical values
    let a1 = Task::from_str(assignment.0).unwrap();
    let a2 = Task::from_str(assignment.1).unwrap();

    match full {
        true => full_overlap(&a1, &a2),
        false => partial_overlap(&a1, &a2),
    }
}

// check if one of the tasks if fully contained in the other
fn full_overlap(t1: &Task, t2: &Task) -> bool {
    (t1.0 >= t2.0 && t1.1 <= t2.1) || (t2.0 >= t1.0 && t2.1 <= t1.1)
}

// check if any of the tasks has any overlap
fn partial_overlap(t1: &Task, t2: &Task) -> bool {
    ((t1.0 >= t2.0 && t1.0 <= t2.1) || (t1.1 >= t2.0 && t1.1 <= t2.1))
        || ((t2.0 >= t1.0 && t2.0 <= t1.1) || (t2.1 >= t1.0 && t2.1 <= t1.1))
}

fn solve(input: &str, full_overlap: bool) -> usize {
    input.lines().filter(|l| overlap(l, full_overlap)).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {}", solve(&inp.to_string(), true));
    println!("part 2: {}", solve(&inp.to_string(), false));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_4_part_1() {
        assert_eq!(
            super::solve(
                r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
                true
            ),
            2
        );
    }

    #[test]
    fn day_4_part_2() {
        assert_eq!(
            super::solve(
                r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
                false
            ),
            4
        );
    }
}
