use std::io::Error;

use aoc_lib::io::input::Input;

// A, X -> Rock
// B, Y -> Paper
// C, Z -> Scissors
fn part_1(tournament: &str) -> u64 {
    tournament.lines().fold(0, |acc: u64, l| {
        if let Some(game) = l.split_once(' ') {
            match game {
                ("A", "X") => acc + 4, // 1 for rock,     3 for draw
                ("A", "Y") => acc + 8, // 2 for paper,    6 for win
                ("A", "Z") => acc + 3, // 3 for scissors, 0 for loss
                ("B", "X") => acc + 1, // 1 for rock,     0 for loss
                ("B", "Y") => acc + 5, // 2 for paper,    3 for draw
                ("B", "Z") => acc + 9, // 3 for scissors, 6 for win
                ("C", "X") => acc + 7, // 1 for rock,     6 for win
                ("C", "Y") => acc + 2, // 2 for paper,    0 for loss
                ("C", "Z") => acc + 6, // 3 for scissors, 3 for draw
                _ => acc,
            }
        } else {
            acc
        }
    })
}

// A -> Rock
// B -> Paper
// C -> Scissors
// X -> Loss
// Y -> Draw
// Z -> Win
fn part_2(tournament: &str) -> u64 {
    tournament.lines().fold(0, |acc: u64, l| {
        if let Some(game) = l.split_once(' ') {
            match game {
                ("A", "X") => acc + 3, // 3 for scissors, 0 for loss
                ("A", "Y") => acc + 4, // 1 for rock,     3 for draw
                ("A", "Z") => acc + 8, // 2 for paper,    6 for win
                ("B", "X") => acc + 1, // 1 for rock,     0 for loss
                ("B", "Y") => acc + 5, // 2 for paper,    3 for draw
                ("B", "Z") => acc + 9, // 3 for scissors, 6 for win
                ("C", "X") => acc + 2, // 2 for paper,    0 for loss
                ("C", "Y") => acc + 6, // 3 for scissors, 3 for draw
                ("C", "Z") => acc + 7, // 1 for rock,     6 for win
                _ => acc,
            }
        } else {
            acc
        }
    })
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
    fn day_2_part_1() {
        assert_eq!(
            super::part_1(
                r#"A Y
B X
C Z"#
            ),
            15
        );
    }

    #[test]
    fn day_2_part_2() {
        assert_eq!(
            super::part_2(
                r#"A Y
B X
C Z"#
            ),
            12
        );
    }
}
