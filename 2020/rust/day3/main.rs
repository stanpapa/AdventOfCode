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
	fn new(input: &String) -> Grid {
		let lines: Vec<&str> = input.lines().collect();
		let width  = lines[0].chars().count();
        let length = lines.len();

		let mut map: HashMap<Point, char> = HashMap::new();
		for y in 0..length {
			let line: Vec<char> = lines[y].chars().collect();
			for (x, c) in line.iter().enumerate().take(width) {
				map.insert(Point(x,y), *c);
			} // (x,c)
		} // y

		Grid {
			grid: map,
			width: width,
			length: length,
		}
	}

	fn traverse(&self, slope_x: &usize, slope_y: &usize) -> usize {
		let mut n_trees = 0;
		let mut x = 0;

		for y in (0..self.length).step_by(*slope_y) {
			if self.grid[&Point(x,y)] == '#' { n_trees += 1; }
			x = (x + slope_x) % self.width;
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
.#..#...#.#".to_string();

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
.#..#...#.#".to_string();

		let grid = Grid::new(&input);

		let slopes = vec!((1,1),
						  (3,1),
						  (5,1),
						  (7,1),
						  (1,2));

		let n = slopes
			.iter()
			.map(|(slope_x, slope_y)| grid.traverse(slope_x, slope_y))
			.product::<usize>();

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

    let slopes = vec![(1,1),
					  (3,1),
					  (5,1),
					  (7,1),
					  (1,2),
    ];

	let n = slopes
		.iter()
		.map(|(slope_x, slope_y)| grid.traverse(slope_x, slope_y))
		.product::<usize>();

	println!("Part2: {}", n);
  
    Ok(())
} 
