use std::{collections::HashSet, io::Error};

use libaoc::io::input::Input;

use rayon::prelude::*;

fn parse_seeds(s: &str) -> Vec<u64> {
    s.split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn construct_maps(maps: &[&str]) -> Vec<HashSet<(u64, u64, u64)>> {
    maps.iter()
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    let map_entry = l
                        .split_whitespace()
                        .map(|i| i.parse().unwrap())
                        .collect::<Vec<u64>>();
                    (map_entry[0], map_entry[1], map_entry[2])
                })
                .collect()
        })
        .collect()
}

fn evaluate_map_single_number(n: &mut u64, map: &HashSet<(u64, u64, u64)>) {
    for (destination_start, source_start, range) in map.iter() {
        if *n < *source_start || *n >= *source_start + *range {
            continue;
        }

        let diff = *n - *source_start;
        *n = *destination_start + diff;
        break;
    }
}

fn evaluate_map(numbers: &mut [u64], map: &HashSet<(u64, u64, u64)>) {
    numbers.par_iter_mut().for_each(|n| {
        evaluate_map_single_number(n, map);
    });
}

fn part_1(input: &str) -> u64 {
    // parse input
    let mut seeds = parse_seeds(input.lines().next().unwrap());
    let maps = construct_maps(&input.split("\n\n").collect::<Vec<&str>>()[1..]);

    maps.iter().for_each(|map| evaluate_map(&mut seeds, map));

    *seeds.iter().min().unwrap()
}

fn part_2(input: &str) -> u64 {
    // parse input
    let seeds = parse_seeds(input.lines().next().unwrap());
    let maps = construct_maps(&input.split("\n\n").collect::<Vec<&str>>()[1..]);

    // brute force
    seeds
        .chunks_exact(2)
        .map(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .into_par_iter()
                .map(|mut i| {
                    maps.iter().for_each(|m| {
                        evaluate_map_single_number(&mut i, m);
                    });
                    i
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 35);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 46);
    }
}
