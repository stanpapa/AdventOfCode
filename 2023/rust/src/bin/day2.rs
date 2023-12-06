use std::{collections::HashMap, io::Error};

use libaoc::io::input::Input;

use regex::Regex;

static RE_PATTERN: &str = r"Game (?<id>\d+):";
static RE_CUBES_PATTERN: &str = r"(?<n>\d+) (?<colour>red|green|blue)";

fn part_1(games: &str) -> u32 {
    // maximum number of cubes per colour
    let max: HashMap<&str, u8> = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    // regex patterns
    let re = Regex::new(RE_PATTERN).unwrap();
    let re_cubes = Regex::new(RE_CUBES_PATTERN).unwrap();

    games.lines().fold(0, |acc, game| {
        // capture game ID
        let Some(caps) = re.captures(game) else {
            return acc;
        };

        // loop over each cube draw (we don't have to distinguish between sets)
        for caps in re_cubes.captures_iter(game) {
            // number of cubes
            let n = caps.name("n").unwrap().as_str().parse::<u8>().unwrap();

            // colour of cubes
            let colour = caps.name("colour").unwrap().as_str();

            // if the current number exceeds the maximum number in the bag, the game is invalid
            if n > max[colour] {
                return acc;
            }
        }

        // add valid game ID for total score
        acc + caps["id"].parse::<u32>().unwrap()
    })
}

fn part_2(games: &str) -> u32 {
    // regex pattern for a cube draw
    let re_cubes = Regex::new(RE_CUBES_PATTERN).unwrap();

    // iterate over games
    games.lines().fold(0, |acc, game| {
        // minimum number of cubes per game
        let mut min: HashMap<&str, u32> = [("red", 0), ("green", 0), ("blue", 0)]
            .into_iter()
            .collect();

        // loop over each cube draw (we don't have to distinguish between sets)
        for caps in re_cubes.captures_iter(game) {
            // number of cubes
            let n = caps.name("n").unwrap().as_str().parse::<u32>().unwrap();

            // colour of cubes
            let colour = caps.name("colour").unwrap().as_str();

            // update minimum number of cubes if current number exceeds the one in the map
            if n > min[colour] {
                min.insert(colour, n);
            }
        }

        // add the numbers of red, green and blue cubes multiplied together
        acc + min.values().product::<u32>()
    })
}
fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(
            super::part_1(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            super::part_2(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
