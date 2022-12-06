use std::{collections::HashSet, error::Error, ops::ControlFlow};

use aoc_lib::io::input::Input;

// break_value is unfortunately only available in the nightly build,
// so for now we have to return the ControlFlow object
fn solve(input: &str, part_1: bool) -> ControlFlow<usize> {
    let window = {
        match part_1 {
            true => 4,
            false => 14,
        }
    };

    // there's probably a better way to do this,
    // but this solution is straightforward
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window)
        .enumerate()
        .try_for_each(|(i, c)| {
            let set: HashSet<&char> = HashSet::from_iter(c.iter());
            if set.len() == window {
                return ControlFlow::Break(i + window);
            }
            ControlFlow::Continue(())
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {:?}", solve(&inp.to_string(), true));
    println!("part 2: {:?}", solve(&inp.to_string(), false));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    #[test]
    fn day_6_part_1() {
        assert_eq!(
            super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", true),
            ControlFlow::Break(7)
        );

        assert_eq!(
            super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz", true),
            ControlFlow::Break(5)
        );

        assert_eq!(
            super::solve("nppdvjthqldpwncqszvftbrmjlhg", true),
            ControlFlow::Break(6)
        );

        assert_eq!(
            super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", true),
            ControlFlow::Break(10)
        );

        assert_eq!(
            super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", true),
            ControlFlow::Break(11)
        );
    }

    #[test]
    fn day_6_part_2() {
        assert_eq!(
            super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", false),
            ControlFlow::Break(19)
        );

        assert_eq!(
            super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz", false),
            ControlFlow::Break(23)
        );

        assert_eq!(
            super::solve("nppdvjthqldpwncqszvftbrmjlhg", false),
            ControlFlow::Break(23)
        );

        assert_eq!(
            super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", false),
            ControlFlow::Break(29)
        );

        assert_eq!(
            super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", false),
            ControlFlow::Break(26)
        );
    }
}
