use std::{collections::HashSet, io::Error};

use libaoc::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Contraption = Grid<char>;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Beam {
    pub location: Coordinate,
    pub direction: Direction,
}

impl Beam {
    pub fn new(location: Coordinate, direction: Direction) -> Self {
        Self {
            location,
            direction,
        }
    }

    pub fn propagate(&mut self) {
        match self.direction {
            Direction::North => self.location += Coordinate::new(0, -1),
            Direction::South => self.location += Coordinate::new(0, 1),
            Direction::East => self.location += Coordinate::new(1, 0),
            Direction::West => self.location += Coordinate::new(-1, 0),
        }
    }
}

fn propagate(contraption: &Contraption, mut beam: Beam, energised: &mut HashSet<Beam>) {
    loop {
        // check if a mirror has been hit
        match contraption.get(&beam.location) {
            Some('\\') => match beam.direction {
                Direction::East => beam.direction = Direction::South,
                Direction::West => beam.direction = Direction::North,
                Direction::South => beam.direction = Direction::East,
                Direction::North => beam.direction = Direction::West,
            },
            Some('/') => match beam.direction {
                Direction::East => beam.direction = Direction::North,
                Direction::West => beam.direction = Direction::South,
                Direction::South => beam.direction = Direction::West,
                Direction::North => beam.direction = Direction::East,
            },
            Some('-') => {
                match beam.direction {
                    Direction::West | Direction::East => {}
                    Direction::South | Direction::North => {
                        // split beams and propagate
                        beam.direction = Direction::East;
                        propagate(contraption, beam.clone(), energised);
                        beam.direction = Direction::West;
                        propagate(contraption, beam.clone(), energised);
                        return;
                    }
                }
            }
            Some('|') => {
                match beam.direction {
                    Direction::East | Direction::West => {
                        // split beams and propagate
                        beam.direction = Direction::North;
                        propagate(contraption, beam.clone(), energised);
                        beam.direction = Direction::South;
                        propagate(contraption, beam.clone(), energised);
                        return;
                    }
                    Direction::North | Direction::South => {}
                }
            }
            _ => {}
        }

        // move beam forward
        beam.propagate();

        // terminate if a beam has reached the boundary of the contraption
        if beam.location.x < 0 || beam.location.x >= contraption.width as isize {
            return;
        }
        if beam.location.y < 0 || beam.location.y >= contraption.length as isize {
            return;
        }

        if energised.contains(&beam) {
            return;
        }

        energised.insert(beam.clone());
    }
}

// fn part_1(input: &str) -> usize {
fn count_energised(contraption: &Contraption, beam: Beam) -> usize {
    let mut energised = HashSet::<Beam>::new();
    energised.insert(beam.clone());

    propagate(contraption, beam, &mut energised);

    energised
        .iter()
        .map(|e| e.location)
        .collect::<HashSet<_>>()
        .len()
}

fn part_2(contraption: &Contraption) -> usize {
    // iterate over the edge of the contraption and check
    // which beam results in the maximum number of tiles
    // being energised. If the max is in the corner, this
    // might not result in a correct answer
    contraption
        .iter_edge()
        .map(|(c, _)| {
            let beam = if c.x == 0 {
                // left edge
                Beam::new(*c, Direction::East)
            } else if c.x == contraption.width as isize - 1 {
                // right edge
                Beam::new(*c, Direction::West)
            } else if c.y == 0 {
                // top edge
                Beam::new(*c, Direction::South)
            } else if c.y == contraption.width as isize - 1 {
                // top edge
                Beam::new(*c, Direction::North)
            } else {
                panic!("Not an edge!");
            };

            count_energised(contraption, beam)
        })
        .max()
        .unwrap()
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();
    let contraption = Contraption::new(&inp);

    println!(
        "{}",
        count_energised(
            &contraption,
            Beam::new(Coordinate::new(0, 0), Direction::East)
        )
    );
    println!("{}", part_2(&contraption));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part_1() {
        let contraption = Contraption::new(&INPUT);
        assert_eq!(
            super::count_energised(
                &contraption,
                Beam::new(Coordinate::new(0, 0), Direction::East)
            ),
            46
        );
    }

    #[test]
    fn part_2() {
        let contraption = Contraption::new(&INPUT);
        assert_eq!(super::part_2(&contraption), 51);
    }
}
