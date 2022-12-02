use std::{collections::HashMap, io::Error};

use aoc_lib::io::input::Input;

/// Rust unfortunately has no native way to declare a const/static HashMap
/// using HashMap::from(), since it's a non-const fn

fn solve(tournament: &str, score_table: &HashMap<&str, u64>) -> u64 {
    tournament
        .lines()
        .map(|l| {
            score_table
                .get(l)
                .unwrap_or_else(|| panic!("Invalid move!"))
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new();

    // A, X -> Rock
    // B, Y -> Paper
    // C, Z -> Scissors
    let score_table_1: HashMap<&str, u64> = HashMap::from([
        ("A X", 4), // 1 for rock,     3 for draw
        ("A Y", 8), // 2 for paper,    6 for win
        ("A Z", 3), // 3 for scissors, 0 for loss
        ("B X", 1), // 1 for rock,     0 for loss
        ("B Y", 5), // 2 for paper,    3 for draw
        ("B Z", 9), // 3 for scissors, 6 for win
        ("C X", 7), // 1 for rock,     6 for win
        ("C Y", 2), // 2 for paper,    0 for loss
        ("C Z", 6), // 3 for scissors, 3 for draw
    ]);

    // A -> Rock
    // B -> Paper
    // C -> Scissors
    // X -> Loss
    // Y -> Draw
    // Z -> Win
    let score_table_2: HashMap<&str, u64> = HashMap::from([
        ("A X", 3), // 3 for scissors, 0 for loss
        ("A Y", 4), // 1 for rock,     3 for draw
        ("A Z", 8), // 2 for paper,    6 for win
        ("B X", 1), // 1 for rock,     0 for loss
        ("B Y", 5), // 2 for paper,    3 for draw
        ("B Z", 9), // 3 for scissors, 6 for win
        ("C X", 2), // 2 for paper,    0 for loss
        ("C Y", 6), // 3 for scissors, 3 for draw
        ("C Z", 7), // 1 for rock,     6 for win
    ]);

    println!("part 1: {}", solve(&inp.to_string(), &score_table_1));
    println!("part 2: {}", solve(&inp.to_string(), &score_table_2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn day_2_part_1() {
        let score_table: HashMap<&str, u64> = HashMap::from([
            ("A X", 4), // 1 for rock,     3 for draw
            ("A Y", 8), // 2 for paper,    6 for win
            ("A Z", 3), // 3 for scissors, 0 for loss
            ("B X", 1), // 1 for rock,     0 for loss
            ("B Y", 5), // 2 for paper,    3 for draw
            ("B Z", 9), // 3 for scissors, 6 for win
            ("C X", 7), // 1 for rock,     6 for win
            ("C Y", 2), // 2 for paper,    0 for loss
            ("C Z", 6), // 3 for scissors, 3 for draw
        ]);

        assert_eq!(
            super::solve(
                r#"A Y
B X
C Z"#,
                &score_table
            ),
            15
        );
    }

    #[test]
    fn day_2_part_2() {
        let score_table: HashMap<&str, u64> = HashMap::from([
            ("A X", 3), // 3 for scissors, 0 for loss
            ("A Y", 4), // 1 for rock,     3 for draw
            ("A Z", 8), // 2 for paper,    6 for win
            ("B X", 1), // 1 for rock,     0 for loss
            ("B Y", 5), // 2 for paper,    3 for draw
            ("B Z", 9), // 3 for scissors, 6 for win
            ("C X", 2), // 2 for paper,    0 for loss
            ("C Y", 6), // 3 for scissors, 3 for draw
            ("C Z", 7), // 1 for rock,     6 for win
        ]);

        assert_eq!(
            super::solve(
                r#"A Y
B X
C Z"#,
                &score_table
            ),
            12
        );
    }
}
