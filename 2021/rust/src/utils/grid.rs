use super::coordinate::Coordinate;
use std::{collections::HashMap, fmt, ops, str::FromStr};

// todo: make iterator
#[derive(Default, Clone)]
pub struct GridMap<T> {
    pub map: HashMap<Coordinate, T>,
    pub length: usize,
    pub width: usize,
}

impl<T> ops::Index<Coordinate> for GridMap<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        match self.map.get(&index) {
            Some(val) => val,
            _ => panic!("Coordinate out of bounds"),
        }
    }
}

impl<T> ops::IndexMut<Coordinate> for GridMap<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        match self.map.get_mut(&index) {
            Some(val) => val,
            _ => panic!("Coordinate out of bounds"),
        }
    }
}

impl<T> GridMap<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    pub fn new(s: String) -> Self {
        let length = s.lines().count();
        let width = s.lines().nth(0).expect("No line").chars().count();

        let mut map: HashMap<Coordinate, T> = Default::default();
        for (y, line) in s.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                map.insert(
                    Coordinate {
                        x: x as i32,
                        y: y as i32,
                    },
                    value.to_string().parse().unwrap(),
                );
            }
        }

        Self { map, length, width }
    }

    pub fn size(&self) -> usize {
        self.length * self.width
    }
}

impl<T> fmt::Display for GridMap<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.length {
            for x in 0..self.width {
                let c = Coordinate {
                    x: x as i32,
                    y: y as i32,
                };

                if let Some(val) = self.map.get(&c) {
                    write!(f, "{}", val.to_string())?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
