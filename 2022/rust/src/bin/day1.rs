use std::io::Error;

use aoc_lib::io::input::Input;

fn part_1(calories: &[Vec<u64>]) -> Option<u64> {
    calories
        .iter()
        .map(|cals| cals.iter().sum())
        .reduce(|accum, item| if accum > item { accum } else { item })
}

fn part_2(calories: &[Vec<u64>]) -> Option<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for cals in calories {
        vec.push(cals.iter().sum::<u64>());
    }

    // sort in decreasing order
    vec.sort_by(|a, b| b.cmp(a));
    Some(vec[0..3].iter().sum())
}

fn main() -> Result<(), Error> {
    let inp = Input::new();

    // convert input to vector of strings
    let s = match inp.to_vec::<String>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // split vector into each elf's inventory
    let mut calories: Vec<Vec<u64>> = Vec::new();
    let mut vec = Vec::new();
    for cal in s {
        if cal == "" {
            calories.push(vec.clone());
            vec.clear();
            continue;
        }
        vec.push(cal.parse::<u64>().unwrap());
    }

    println!("{:?}", part_1(&calories));
    println!("{:?}", part_2(&calories));

    Ok(())
}
