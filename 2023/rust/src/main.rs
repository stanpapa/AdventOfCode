use libaoc::Day;

use std::process::Command;

fn run(day: &Day) {
    let binary = if cfg!(debug_assertions) {
        format!("./target/debug/{}", day)
    } else {
        format!("./target/release/{}", day)
    };
    let input = format!("../input/{}.txt", day);

    Command::new(binary)
        .arg(input)
        .status()
        .expect("Something went inside run()");
}

fn main() {
    for puzzle in Day::iter() {
        println!("=================\n{}", puzzle);
        run(puzzle);
    }
}
