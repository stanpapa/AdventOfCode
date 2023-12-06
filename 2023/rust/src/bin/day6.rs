use std::io::Error;

use libaoc::io::input::Input;

use rayon::prelude::*;

fn n_ways_to_beat(t_max: u64, d_min: u64) -> u64 {
    // v = t_charge
    // distance = v * t_remaining = t_charge * (t_max - t_charge)

    // brute force
    (0..t_max)
        .into_par_iter()
        .filter(|t| (t_max - t) * t > d_min)
        .count() as u64

    // solve quadratic equation: -t**2 + t_max * t - d_min = 0
    // let discriminant = ((t_max * t_max - 4 * d_min) as f64).sqrt();
    // let t0 = (t_max as f64 - discriminant) / 2.;
    // let t1 = (t_max as f64 + discriminant) / 2.;

    // // round down (and take care of corner case where the solution is an integer)
    // let t_right = if (t1 - t1.round()).abs() < 1e-10 {
    //     // this is an integer, so we have to shift up
    //     (t1 - 1.).floor() as u64
    // } else {
    //     t1.floor() as u64
    // };

    // // round up (and take care of corner case where the solution is an integer)
    // let t_left = if (t0 - t0.round()).abs() < 1e-10 {
    //     // this is an integer, so we have to shift up
    //     (t0 + 1.).ceil() as u64
    // } else {
    //     t0.ceil() as u64
    // };

    // // ways to beat is the range between the 2 solution points
    // t_right - t_left + 1
}

fn part_1(input: &str) -> u64 {
    // parse input
    let (time, distance_str) = input.split_once('\n').unwrap();
    let t = time
        .split_whitespace()
        .skip(1)
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u64>>();
    let d = distance_str
        .split_whitespace()
        .skip(1)
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u64>>();
    let races = t.into_iter().zip(d).collect::<Vec<(u64, u64)>>();

    // iterate over races and multiply the numbers of ways the record can be beat
    races
        .iter()
        .map(|(t_max, distance_min)| {
            // count how many ways the number can be beat for a specific race
            n_ways_to_beat(*t_max, *distance_min)
        })
        .product()
}

fn part_2(input: &str) -> u64 {
    // parse input -> concatenate time and distance into a single number, respectively
    let (time, distance_str) = input.split_once('\n').unwrap();
    let t_max = time
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let d_min = distance_str
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    // solution is to only calculate number of ways to beat for a single long race
    n_ways_to_beat(t_max, d_min)
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 288);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 71503);
    }
}
