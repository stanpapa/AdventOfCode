use std::io::Error;

use aoc_2021_rust::utils::input;

fn is_line_corrupt(line: &String) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        // push opening bracket to stack
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                continue;
            }
            _ => (),
        };

        // check if brackets match
        match (stack.pop(), c) {
            (Some('('), ')') => (),
            (Some('['), ']') => (),
            (Some('{'), '}') => (),
            (Some('<'), '>') => (),
            (_, ')') => return 3,
            (_, ']') => return 57,
            (_, '}') => return 1197,
            (_, '>') => return 25137,
            _ => unreachable!(),
        };
    }

    0
}

// total syntax error score for all corrupted lines
fn part_1(input: &Vec<String>) -> usize {
    input.iter().map(|line| is_line_corrupt(line)).sum()
}

fn completion_string(line: &String) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        // push opening bracket to stack
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                continue;
            }
            _ => (),
        };

        // characters match, so remove them from stack and continue
        stack.pop();
    }

    // stack now only contains unclosed brackets
    let score = stack.iter().rev().fold(0, |score, c| match c {
        '(' => score * 5 + 1,
        '[' => score * 5 + 2,
        '{' => score * 5 + 3,
        '<' => score * 5 + 4,
        _ => unreachable!(),
    });

    score
}

fn part_2(input: &Vec<String>) -> usize {
    let incomplete_lines = input
        .iter()
        .filter(|line| is_line_corrupt(line) == 0)
        .collect::<Vec<_>>();

    let mut scores = incomplete_lines
        .iter()
        .map(|line| completion_string(line))
        .collect::<Vec<_>>();

    // the winner is found by sorting all of the scores and then taking the middle score
    scores.sort();
    scores[scores.len() / 2]
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
