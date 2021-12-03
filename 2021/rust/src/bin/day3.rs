use std::{fmt, io::Error, str::FromStr};

use aoc_2021_rust::utils::input;

// we use a struct for storing binary numbers to ensure that all numbers are valid
#[derive(Clone, Debug)]
struct BinaryNumber {
    num: Vec<bool>,
}

// convert a string to BinaryNumber by using parse()
impl FromStr for BinaryNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        for c in s.chars() {
            match c {
                '0' => Ok(v.push(false)),
                '1' => Ok(v.push(true)),
                _ => Err("Not a valid binary number!"),
            }?;
        }

        Ok(BinaryNumber { num: v })
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for BinaryNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl BinaryNumber {
    // convert a BinaryNumber to a decimal number using powers of 2
    // others use shift operators for this (<<,>>), but I don't understand it
    fn to_decimal(&self) -> u64 {
        let mut decimal = 0;

        // example:
        // 1101
        // is converted using 1*2^3 + 1*2^2 + 0*2^1 + 1*2^0
        let l = self.num.len() - 1;
        for i in 0..=l {
            decimal += self.num[i] as u64 * 2_u64.pow((l - i) as u32);
        }
        decimal
    }

    // surely, there is a better way
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in &self.num {
            match i {
                true => s.push('1'),
                false => s.push('0'),
            }
        }
        s
    }

    // invert a binary number by swapping 0's for 1's and vice versa
    fn invert(&self) -> BinaryNumber {
        BinaryNumber {
            num: self.num.iter().map(|b| !b).collect(),
        }
    }
}

// count the number of bit occurrences per position
fn count_bit(numbers: &Vec<BinaryNumber>, pos: usize, bit: bool) -> u32 {
    numbers.iter().filter(|x| x.num[pos] == bit).count() as u32
}

fn part_1(binary_numbers: &Vec<BinaryNumber>) -> u64 {
    // the gamma rate is constructed from the most common bits per position
    let mut gamma = String::new();
    for i in 0..binary_numbers[0].num.len() {
        if count_bit(binary_numbers, i, true) > count_bit(binary_numbers, i, false) {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }

    let gamma = gamma.parse::<BinaryNumber>().expect("Not a binary number");

    // the epsilon rate is the inverted gamma binary number
    let epsilon = gamma.invert();

    // power consumption
    gamma.to_decimal() * epsilon.to_decimal()
}

fn calc_rating(binary_numbers: &Vec<BinaryNumber>, bit: bool) -> BinaryNumber {
    let mut filtered = binary_numbers.to_vec();
    let mut pos = 0;

    while filtered.len() > 1 {
        let count = {
            if bit {
                // oxygen most recurring bit. Equal number -> 1
                count_bit(&filtered, pos, bit) >= count_bit(&filtered, pos, !bit)
            } else {
                // co2 least recurring bit. Equal number -> 0
                count_bit(&filtered, pos, bit) > count_bit(&filtered, pos, !bit)
            }
        };
        // filter based criteria
        filtered = filtered
            .into_iter()
            .filter(|num| num.num[pos] == count)
            .collect();
        pos += 1;
    }

    filtered[0].clone()
}

fn part_2(binary_numbers: &Vec<BinaryNumber>) -> u64 {
    let oxygen_generator_rating = calc_rating(binary_numbers, true);
    let co2_scrubber_rating = calc_rating(binary_numbers, false);

    // life support rating
    oxygen_generator_rating.to_decimal() * co2_scrubber_rating.to_decimal()
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_vec()?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
