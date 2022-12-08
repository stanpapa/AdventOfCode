use crate::grid::coordinate::Coordinate;
use std::{
    collections::{hash_map::IntoIter, HashMap},
    fmt, ops,
    str::FromStr,
};

#[derive(Default, Clone)]
pub struct Grid<T> {
    map: HashMap<Coordinate, T>,
    pub length: usize,
    pub width: usize,
}

impl<T> Grid<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    pub fn new(s: &str) -> Self {
        let length = s.lines().count();
        let width = s.lines().nth(0).expect("No line").chars().count();

        let mut map: HashMap<Coordinate, T> = Default::default();
        for (y, line) in s.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                map.insert(
                    Coordinate::new(x as isize, y as isize),
                    value.to_string().parse().unwrap(),
                );
            }
        }

        Self { map, length, width }
    }

    pub fn size(&self) -> usize {
        self.length * self.width
    }

    pub fn get(&self, key: &Coordinate) -> Option<&T> {
        self.map.get(key)
    }

    pub fn contains_key(&self, key: &Coordinate) -> bool {
        self.map.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coordinate, &T)> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Coordinate, &mut T)> {
        self.map.iter_mut()
    }

    pub fn len_edge(&self) -> usize {
        self.width * 2 + (self.length - 2) * 2
    }

    pub fn is_edge(&self, coord: &Coordinate) -> bool {
        coord.x == 0
            || coord.x == self.width as isize - 1
            || coord.y == 0
            || coord.y == self.length as isize - 1
    }

    pub fn iter_inner(&self) -> impl Iterator<Item = (&Coordinate, &T)> {
        self.map.iter().filter(|(coord, _)| !self.is_edge(coord))
    }
}

// Return reference to value in map by using square brackets
// Example: GridMap[coord] -> &value
impl<T> ops::Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        match self.map.get(&index) {
            Some(val) => val,
            _ => panic!("Coordinate out of bounds"),
        }
    }
}

// Return mutable reference to value in map by using square brackets
// Example: GridMap[coord] -> &mut value
impl<T> ops::IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        match self.map.get_mut(&index) {
            Some(val) => val,
            _ => panic!("Coordinate out of bounds"),
        }
    }
}

// todo: make iterator work with generic T
// impl IntoIterator for GridMap<u8> {
//     type Item = (Coordinate, u8);
//     type IntoIter = IntoIter<Coordinate, u8>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.map.into_iter()
//     }
// }

// allow formatted print of grid using `{}`
impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.length {
            for x in 0..self.width {
                let c = Coordinate::new(x as isize, y as isize);

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