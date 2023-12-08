use std::{collections::HashMap, io::Error};

use libaoc::io::input::Input;

use regex::Regex;

fn construct_graph(vertices: &str) -> HashMap<&str, (&str, &str)> {
    // define regex to find vertices
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    // loop over all lines to contruct the graph / linked list
    re.captures_iter(vertices)
        .map(|caps| {
            (
                caps.get(1).unwrap().as_str(),
                (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
            )
        })
        .collect::<HashMap<&str, (&str, &str)>>()
}

fn part_1(input: &str) -> usize {
    // parse the directions and construct the graph
    let (directions, vertices) = input.split_once("\n\n").unwrap();
    let graph = construct_graph(vertices);

    // define counter and starting point
    let mut counter = 0;
    let mut vertex_current = "AAA";

    // follow the directions until you end up at vertex "ZZZ"
    loop {
        let index = counter % directions.len();

        vertex_current = match directions.as_bytes()[index] {
            b'L' => graph[vertex_current].0,
            b'R' => graph[vertex_current].1,
            _ => panic!("Direction not found!"),
        };

        counter += 1;
        if vertex_current == "ZZZ" {
            break;
        }
    }

    counter
}

fn part_2(input: &str) -> usize {
    // parse the directions and construct the graph
    let (directions, vertices) = input.split_once("\n\n").unwrap();
    let graph = construct_graph(vertices);

    // find all vertices that end with 'A'. There are the starting points
    let vertices_current = graph
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();

    // define greatest common divisor
    let gcd = |mut a: usize, mut b: usize| {
        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        a
    };

    // define least common multiple
    let lcm = |a: usize, b: usize| (a * b) / gcd(a, b);

    // for each starting vertex, find the shortest path to a vertex ending with 'Z'
    let path_lengths = vertices_current
        .iter()
        .map(|&v| {
            let mut counter = 0;
            let mut current = v;
            loop {
                let index = counter % directions.len();

                current = match directions.as_bytes()[index] {
                    b'L' => graph[current].0,
                    b'R' => graph[current].1,
                    _ => panic!("Direction not found!"),
                };

                counter += 1;
                if current.ends_with('Z') {
                    break;
                }
            }
            counter
        })
        .collect::<Vec<usize>>();

    // all paths converge for the least common multiple of all paths
    path_lengths
        .iter()
        .skip(1)
        .fold(path_lengths[0], |acc, &x| lcm(acc, x))
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
                r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
        assert_eq!(
            super::part_1(
                r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            super::part_2(
                r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        )
    }
}
