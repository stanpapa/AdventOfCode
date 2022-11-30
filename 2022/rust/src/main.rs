use aoc_lib::Day;
use std::process::Command;

#[cfg(debug_assertions)]
fn run(day: Day) {
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
    for puzzle in Day::iter() {
        println!("=================\n{}", puzzle);
        //        run(puzzle);
    }
}
