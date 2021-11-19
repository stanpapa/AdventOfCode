use std::env;
use std::fs;
use std::io::{Error};
use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

struct Grid {
	grid: HashMap<Point, char>,
	width: usize,
	length: usize,
}

impl Grid {
	fn new(input: &str) -> Grid {

		// is there a better way to initialize tnis map? There must be
		let mut tmp: HashMap<Point, char> = HashMap::new();
		let mut y = 0;
		for row in input.lines() {
			let mut x = 0;
			for column in row.chars() {
				tmp.insert(Point(x,y), column);
				x += 1;
			}
			y += 1;
		}

		Grid {
			grid: tmp,
			width: input.lines().nth(0).unwrap().len(),
			length: input.lines().count(),
		}
	}

	fn traverse(&self, slope_x: &usize, slope_y: &usize) -> usize {
		let mut n_trees = 0;
		let mut x = 0;

		for y in (0..self.length).step_by(*slope_y) {
			if self.grid[&Point(x,y)] == '#' { n_trees += 1; }
			x += slope_x;
			if x >= self.width { x -= self.width }
		}

		n_trees
	}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
		let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

		let grid = Grid::new(&input);

    	assert_eq!(7, grid.traverse(&3,&1));
    }

    #[test]
    fn part2() {
		let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

		let grid = Grid::new(&input);

		let mut n = grid.traverse(&1, &1);
		n *= grid.traverse(&3, &1);
		n *= grid.traverse(&5, &1);
		n *= grid.traverse(&7, &1);
		n *= grid.traverse(&1, &2);

		assert_eq!(336, n);
    }
}


fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();
  
    if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }
    
    // read input from file
    let filename = &args[1];
    let input = fs::read_to_string(filename)?;

    let grid = Grid::new(&input);

    println!("Part1: {:?}", grid.traverse(&3,&1));

    let mut n = grid.traverse(&1, &1);
	n *= grid.traverse(&3, &1);
	n *= grid.traverse(&5, &1);
	n *= grid.traverse(&7, &1);
	n *= grid.traverse(&1, &2);

	println!("Part2: {}", n);
  
    Ok(())
} 
