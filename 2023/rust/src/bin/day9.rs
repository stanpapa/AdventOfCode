use std::io::Error;

use libaoc::io::input::Input;

fn parse_history(history: &str) -> Vec<i64> {
    history
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn step_difference(history: &[i64]) -> Vec<i64> {
    history.windows(2).map(|i| i[1] - i[0]).collect()
}

fn part_1(histories: &str) -> i64 {
    histories
        .lines()
        .map(|history| {
            // convert string into Vec<u64>
            let mut current = parse_history(history);

            // accumulate extrapolated value
            let mut prediction = *current.last().unwrap();

            // iterate until all numbers are the same
            loop {
                if current.iter().all(|&i| i == current[0]) {
                    break;
                }

                // construct new row
                current = step_difference(&current);

                // accumulate extrapolated value
                prediction += *current.last().unwrap();
            }

            // extrapolated value is equal to the sum of all final numbers in a row
            prediction
        })
        .sum()
}

fn part_2(histories: &str) -> i64 {
    histories
        .lines()
        .map(|history| {
            // convert string into Vec<u64>
            let mut current = parse_history(history);

            // accumulate first values in the rows
            let mut h = vec![*current.first().unwrap()];

            // iterate until all numbers are the same
            loop {
                if current.iter().all(|&i| i == current[0]) {
                    break;
                }

                // construct new row
                current = step_difference(&current);

                // add first value of current row to list
                h.push(*current.first().unwrap());
            }

            // extrapolate backwards
            h.iter().rev().fold(0, |acc, i| i - acc)
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 114);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 2);
    }
}
