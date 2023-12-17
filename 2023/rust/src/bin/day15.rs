use std::{collections::HashMap, io::Error};

use libaoc::io::input::Input;

use regex::Regex;

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, ch| (acc + ch as u32) * 17 % 256)
}

fn part_1(input: &str) -> u32 {
    input.replace('\n', "").split(',').map(hash).sum()
}

fn part_2(input: &str) -> u32 {
    let re = Regex::new(r"(?<label>[a-zA-Z]+)(?<op>[=-])(?<focal>[1-9])?").unwrap();
    let mut map = HashMap::<u32, Vec<(&str, u8)>>::new();

    // iterate over all steps
    re.captures_iter(input).for_each(|caps| {
        // retrieve label (first 2 letters of step and convert it into a HASH value)
        let label = caps.name("label").unwrap().as_str();
        let label_hash = hash(label);

        // depending on the operation (=/-), set a value or remove one, respectively
        match caps.name("op").unwrap().as_str() {
            "=" => {
                // retrieve current focal length
                let focal = caps.name("focal").unwrap().as_str().parse::<u8>().unwrap();

                map.entry(label_hash)
                    .and_modify(|b| {
                        // if the label is already present in the Box, replace the focal length with the current value
                        for lens in b.iter_mut() {
                            if lens.0 == label {
                                lens.1 = focal;
                                return;
                            }
                        }

                        // if the label does not exist yet, creat a new entry
                        b.push((label, focal));
                    })
                    .or_insert(vec![(label, focal)]); // add entry if the Box is empty
            }
            "-" => {
                if let Some(b) = map.get_mut(&label_hash) {
                    // if the current lens label can be found in a Box, remove it
                    for i in 0..b.len() {
                        if b[i].0 == label {
                            b.remove(i);
                            break;
                        }
                    }
                }
            }
            _ => panic!("Operation invalid!"),
        }
    });

    // calculate focussing power
    map.iter()
        .map(|(i, b)| {
            //

            (i + 1)
                * b.iter()
                    .enumerate()
                    .map(|(j, (_, focal))| (j + 1) * *focal as usize)
                    .sum::<usize>() as u32
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
    const INPUT: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1() {
        assert_eq!(super::hash("HASH"), 52);
        assert_eq!(super::part_1(INPUT), 1320);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 145);
    }
}
