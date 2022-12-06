pub mod io;

use std::{fmt, slice::Iter};

pub enum Error {
    IO,
    Parse,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Day {
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
    //    day7,
    //    day8,
    //    day9,
    //    day10,
    //    day11,
    //    day12,
    //    day13,
    //    day14,
    //    day15,
    //    day16,
    //    day17,
    //    day18,
    //    day19,
    //    day20,
    //    day21,
    //    day22,
    //    day23,
    //    day24,
    //    day25,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Day {
    pub fn iter() -> Iter<'static, Day> {
        static DAY: [Day; 6] = [
            Day::day1,
            Day::day2,
            Day::day3,
            Day::day4,
            Day::day5,
            Day::day6,
        ];
        DAY.iter()
    }
}
