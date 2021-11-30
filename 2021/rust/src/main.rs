use std::process::Command;

#[cfg(debug_assertions)]
fn run (day: &str) {
    let binary = format!("./target/debug/{}",day);
    let input = format!("./input/{}.txt", day);

    Command::new(binary)
        .arg(input)
        .spawn()
        .expect("Something went inside run()");
}

#[cfg(not(debug_assertions))]
fn run (day: &str) {
    let binary = format!("./target/release/{}",day);
    let input = format!("./input/{}.txt", day);

    Command::new(binary)
        .arg(input)
        .spawn()
        .expect("Something went inside run()");
}

fn main() {

    let puzzles = vec![
        "day1",
    ];

    for puzzle in puzzles {
        run(puzzle);
    }
}
