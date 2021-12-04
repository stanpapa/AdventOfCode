use std::{collections::HashMap, fmt, io::Error, str::FromStr};

use aoc_2021_rust::utils::input;

// The bingo board is always a 5x5 square
// the board itself is represented as a HashMap,
// where <(x,y),(number, marked)>
#[derive(Clone)]
struct BingoBoard {
    width: u8,
    board: HashMap<(u8, u8), (u32, bool)>,
}

impl FromStr for BingoBoard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = HashMap::new();

        for i in 0..5 {
            let line = s.lines().nth(i).expect("nope");
            for j in 0..5 {
                let number = line
                    .split_whitespace()
                    .nth(j)
                    .expect("Out of bounds")
                    .parse::<u32>()
                    .expect("Not a number");
                board.insert((i as u8, j as u8), (number, false));
            }
        }

        Ok(BingoBoard { width: 5, board })
    }
}

// print BingoBoard as a nice grid
impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.width {
            for j in 0..self.width {
                let pos = &(i, j);
                let n = format!("({},{}) ", self.board[pos].0, self.board[pos].1);
                s.push_str(&n);
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl BingoBoard {
    fn mark(&mut self, num: &u32) {
        for i in 0..self.width {
            for j in 0..self.width {
                let pos = &(i, j);
                if self.board[pos].0 == *num {
                    *self.board.get_mut(pos).unwrap() = (*num, true);
                }
            }
        }
    }

    fn check_row_or_column(&self, pos: u8, check_row: bool) -> bool {
        let mut count: u8 = 0;
        for i in 0..self.width {
            if check_row {
                count += self.board[&(pos, i)].1 as u8;
            } else {
                count += self.board[&(i, pos)].1 as u8;
            }
        }

        if self.width == count {
            return true;
        }
        false
    }

    fn check_bingo(&self) -> bool {
        for check_row in [true, false] {
            for row in 0..self.width {
                if self.check_row_or_column(row, check_row) {
                    return true;
                }
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.board
            .values()
            .filter(|value| value.1 == false)
            .map(|value| value.0)
            .sum()
    }
}

// check which board wins first and calculate its final score
fn part_1(numbers: &Vec<u32>, bingo_boards: &Vec<BingoBoard>) -> u32 {
    let mut boards = bingo_boards.to_vec();

    for num in numbers {
        for board in &mut boards {
            board.mark(num);

            // return final score when first board has bingo
            if board.check_bingo() {
                return board.sum_unmarked() * *num as u32;
            }
        }
    }

    0
}

// determine which board wins last and calculate its final score
fn part_2(numbers: &Vec<u32>, bingo_boards: &Vec<BingoBoard>) -> u32 {
    let mut boards_remaining = bingo_boards.to_vec(); // clone, no borrowing

    for num in numbers {
        // mark the boards
        for board in &mut boards_remaining {
            board.mark(num);
        }

        // return final score when last remaining board has bingo
        if boards_remaining.len() == 1 && boards_remaining[0].check_bingo() {
            return boards_remaining[0].sum_unmarked() * *num as u32;
        }

        // filter out all boars that bingo
        boards_remaining = boards_remaining
            .into_iter()
            .filter(|board| !board.check_bingo())
            .collect();
    }

    0
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_string()?;

    // split input by blank lines
    let split: Vec<_> = input.split("\n\n").collect();

    // first line is numbers to check
    let numbers: Vec<u32> = split[0]
        .split(",")
        .map(|n| n.parse().expect("Unable to parse number"))
        .collect();

    // construct the bingo boards
    let mut bingo_boards = Vec::new();
    for i in 1..split.len() {
        bingo_boards.push(
            split[i]
                .parse::<BingoBoard>()
                .expect("Not a valid bingo board"),
        );
    }

    println!("Part 1: {}", part_1(&numbers, &bingo_boards));
    println!("Part 2: {}", part_2(&numbers, &bingo_boards));

    Ok(())
}
