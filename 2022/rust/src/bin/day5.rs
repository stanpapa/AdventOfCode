use std::{collections::VecDeque, error::Error, ops::ControlFlow};

use aoc_lib::io::input::Input;

fn cratemover_9000(stacks: &mut Vec<VecDeque<char>>, amount: usize, from: usize, to: usize) {
    for _i in 0..amount {
        // Rust forces me to store the crate in a local variable,
        // because I can't mutably borrow twice
        let moved = stacks[from].pop_front().unwrap();
        stacks[to].push_front(moved);
    }
}

fn cratemover_9001(stacks: &mut Vec<VecDeque<char>>, amount: usize, from: usize, to: usize) {
    for i in (0..amount).rev() {
        let moved = stacks[from].remove(i).unwrap();
        stacks[to].push_front(moved);
    }
}

fn solve(input: &str, part_1: bool) -> String {
    // calculate number of stacks
    let n = (input.lines().nth(0).expect("Line not found").len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); n];
    let mut begin = 0;

    // construct stacks
    input.lines().enumerate().try_for_each(|(j, l)| {
        // once we reach the empty line, the stacks are complete
        if l.len() == 0 {
            begin = j + 1;
            return ControlFlow::Break((j, l));
        }

        (0..n).for_each(|i| match l.chars().nth(4 * i + 1) {
            Some(c) => {
                if c.is_alphabetic() && c.is_uppercase() {
                    stacks[i].push_back(c);
                }
            }
            None => (),
        });
        ControlFlow::Continue(())
    });

    // move the crates
    input
        .lines()
        .enumerate()
        .skip_while(|(i, _)| i < &begin)
        .for_each(|(_, l)| {
            let split = l.split(' ').collect::<Vec<&str>>();
            let amount = split[1].parse::<usize>().unwrap();
            let from = split[3].parse::<usize>().unwrap() - 1;
            let to = split[5].parse::<usize>().unwrap() - 1;
            match part_1 {
                true => cratemover_9000(&mut stacks, amount, from, to),
                false => cratemover_9001(&mut stacks, amount, from, to),
            }
        });

    // return top of the stacks as &str
    stacks
        .iter()
        .map(|q| q.front().unwrap())
        .collect::<String>()
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
    fn day_5_part_1() {
        assert_eq!(
            super::solve(
                r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
                true
            ),
            "CMZ"
        );
    }

    #[test]
    fn day_5_part_2() {
        assert_eq!(
            super::solve(
                r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
                false
            ),
            "MCD"
        );
    }
}
