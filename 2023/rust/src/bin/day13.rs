use std::io::Error;

use libaoc::io::input::Input;

fn transpose(s: &str) -> String {
    // Split the multiline string into lines
    let lines: Vec<&str> = s.lines().collect();

    // Transpose the characters within each line
    let mut transposed_lines: Vec<String> = Vec::new();

    // Find the maximum line length
    let max_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    for i in 0..max_length {
        let transposed_line: String = lines
            .iter()
            .filter_map(|line| line.chars().nth(i))
            .collect();
        transposed_lines.push(transposed_line);
    }

    // Join the transposed lines to form the final result
    transposed_lines.join("\n")
}

fn find_reflection(pattern: &str, exclude: usize) -> Option<(usize, usize)> {
    let lines = pattern.lines().collect::<Vec<&str>>();

    for (i, w) in lines.windows(2).enumerate() {
        // move on if the current 2 lines are not the same
        // or if we exclude the line
        if w[0] != w[1] || i == exclude {
            continue;
        }

        // if the first line is the same as the second,
        // that qualifies as a reflection line
        if i == 0 {
            return Some((1, 0));
        }

        // we have found a potential reflection line
        // check if it actually is one
        let mut j = 1;
        loop {
            // if we have reached one of the ends, it means we
            // have found a reflection line
            if i < j || lines.get(i + 1 + j).is_none() {
                return Some((i + 1, i));
            }

            // not a reflection plane, since it doesn't propagate through
            // the entire pattern/pattern edge
            if lines[i - j] != lines[i + 1 + j] {
                break;
            }

            // move outwards
            j += 1;
        }
    }

    None
}

fn summary(pattern: &str, exclude: (usize, usize)) -> Option<(usize, (usize, usize))> {
    if let Some((n, row)) = find_reflection(pattern, exclude.0) {
        return Some((n * 100, (row, usize::MAX)));
    }

    if let Some((n, col)) = find_reflection(&transpose(pattern), exclude.1) {
        return Some((n, (usize::MAX, col)));
    }

    None
}

/// I really wish there was a nicer way to change a single character in a multiline String
fn remove_smudge(pattern: &str, r: usize, c: usize) -> String {
    let mut lines = pattern
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    match lines[r].chars().nth(c) {
        Some('.') => lines[r].replace_range(c..=c, "#"),
        Some('#') => lines[r].replace_range(c..=c, "."),
        _ => panic!("Cannot remove smudge"),
    }

    lines.join("\n")
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| match summary(pattern, (usize::MAX, usize::MAX)) {
            Some(sum) => sum.0,
            None => panic!("No mirror plane found"),
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| match summary(pattern, (usize::MAX, usize::MAX)) {
            Some(sum) => {
                // remove smudges in a brute-force manner
                for r in 0..pattern.lines().count() {
                    for c in 0..pattern
                        .lines()
                        .next()
                        .map_or(0, |line| line.chars().count())
                    {
                        // remove smudge
                        let pattern_new = remove_smudge(pattern, r, c);

                        // check if new pattern contains a new reflection line
                        if let Some(summary) = summary(&pattern_new, sum.1) {
                            return summary.0;
                        }
                    }
                }
                panic!("No mirror plane found after removing smudges")
            }
            None => panic!("No initial mirror plane found"),
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
    const INPUT: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 405);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 400);
    }
}
