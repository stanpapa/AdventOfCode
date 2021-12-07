use std::process::Command;

#[cfg(debug_assertions)]
fn run(day: &str) {
    let binary = format!("./target/debug/{}", day);
    let input = format!("./input/{}.txt", day);

    Command::new(binary)
        .arg(input)
        .status()
        .expect("Something went inside run()");
}

#[cfg(not(debug_assertions))]
fn run(day: &str) {
    let binary = format!("./target/release/{}", day);
    let input = format!("./input/{}.txt", day);

    Command::new(binary)
        .arg(input)
        .status()
        .expect("Something went inside run()");
}

fn main() {
    let puzzles = vec!["day1", "day2", "day3", "day4", "day5", "day6", "day7"];

    for puzzle in puzzles {
        println!("=================\n{}", puzzle);
        run(puzzle);
    }
}
