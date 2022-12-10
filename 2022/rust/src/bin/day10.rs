use std::error::Error;

use aoc_lib::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type CRT = Grid<char>;

#[derive(Debug)]
struct Sprite(Coordinate, Coordinate, Coordinate);

impl Sprite {
    fn new() -> Self {
        Self(
            Coordinate::new(0, 0),
            Coordinate::new(1, 0),
            Coordinate::new(2, 0),
        )
    }

    fn shift(&mut self, coord: &Coordinate) {
        self.0 += coord;
        self.1 += coord;
        self.2 += coord;
    }

    fn overlap(&self, other: &Coordinate) -> bool {
        self.0 == *other || self.1 == *other || self.2 == *other
    }
}

fn check_cycle(signal_strengths: &mut Vec<isize>, cycle: &mut isize, register: isize) {
    if *cycle % 40 == 20 {
        signal_strengths.push(*cycle * register);
    }
    *cycle += 1;
}

fn part_1(input: &str) -> isize {
    let mut register = 1;
    let mut cycle = 1;
    let mut signal_strengths = Vec::new();

    input.lines().for_each(|l| match l {
        "noop" => {
            check_cycle(&mut signal_strengths, &mut cycle, register);
        }
        _ => {
            if let Some((_, x)) = l.split_once(' ') {
                check_cycle(&mut signal_strengths, &mut cycle, register);
                check_cycle(&mut signal_strengths, &mut cycle, register);
                register += x.parse::<isize>().unwrap();
            };
        }
    });

    signal_strengths.iter().sum()
}

fn draw_pixel(crt: &mut CRT, sprite: &mut Sprite, cycle: &mut usize) {
    let pos = Coordinate::new(((*cycle - 1) % 40) as isize, ((*cycle - 1) / 40) as isize);

    if sprite.overlap(&pos) {
        crt[pos] = '#';
    }

    // shift to new line
    if *cycle % 40 == 0 {
        sprite.shift(&Coordinate::new(0, 1));
    }

    *cycle += 1;
}

fn part_2(input: &str) -> String {
    let mut cycle = 1;
    let mut crt = CRT::new_with_value(40, 6, '.');
    let mut sprite = Sprite::new();

    input.lines().for_each(|l| match l {
        "noop" => {
            draw_pixel(&mut crt, &mut sprite, &mut cycle);
        }
        _ => {
            if let Some((_, x)) = l.split_once(' ') {
                draw_pixel(&mut crt, &mut sprite, &mut cycle);
                draw_pixel(&mut crt, &mut sprite, &mut cycle);
                sprite.shift(&Coordinate::new(x.parse::<isize>().unwrap(), 0));
            };
        }
    });

    crt.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {}", part_1(&inp.to_string()));
    println!("part 2:\n{}", part_2(&inp.to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_10_part_1() {
        assert_eq!(
            super::part_1(
                r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#
            ),
            13140
        );
    }

    #[test]
    fn day_10_part_2() {
        assert_eq!(
            super::part_2(
                r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#
            ),
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
        );
    }
}
