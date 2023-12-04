use std::io::Error;

use libaoc::io::input::Input;

fn solve(calibration: &str) -> u32 {
    calibration.lines().fold(0, |acc, l| {
        let c0 = l
            .chars()
            .find(|&c| c > '0' && c <= '9')
            .unwrap()
            .to_digit(10)
            .unwrap();

        let c1 = l
            .chars()
            .rfind(|&c| c > '0' && c <= '9')
            .unwrap()
            .to_digit(10)
            .unwrap();

        acc + c0 * 10 + c1
    })
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", solve(&inp));
    println!(
        "{}",
        solve(
            &inp.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        )
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                &r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
            ),
            281
        );
    }
}
