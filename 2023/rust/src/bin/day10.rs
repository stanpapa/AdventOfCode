use std::io::Error;

use libaoc::{
    grid::{coordinate::Coordinate, grid::Grid},
    io::input::Input,
};

type Tiles = Grid<char>;

/// find 1 of 2 possible directions
/// up -> |, 7, F
/// down -> |, J, L
/// right -> -, J, 7
/// left -> -, F, L
fn find_first_move(tiles: &Tiles, start: Coordinate) -> Coordinate {
    if Some(&'|') == tiles.get(&(start + Coordinate::new(0, -1)))
        || Some(&'7') == tiles.get(&(start + Coordinate::new(0, -1)))
        || Some(&'F') == tiles.get(&(start + Coordinate::new(0, -1)))
    {
        // up
        start + Coordinate::new(0, -1)
    } else if Some(&'|') == tiles.get(&(start + Coordinate::new(0, 1)))
        || Some(&'J') == tiles.get(&(start + Coordinate::new(0, 1)))
        || Some(&'L') == tiles.get(&(start + Coordinate::new(0, 1)))
    {
        // down
        start + Coordinate::new(0, 1)
    } else if Some(&'-') == tiles.get(&(start + Coordinate::new(1, 0)))
        || Some(&'J') == tiles.get(&(start + Coordinate::new(1, 0)))
        || Some(&'7') == tiles.get(&(start + Coordinate::new(1, 0)))
    {
        // right
        start + Coordinate::new(1, 0)
    } else if Some(&'-') == tiles.get(&(start + Coordinate::new(-1, 0)))
        || Some(&'F') == tiles.get(&(start + Coordinate::new(-1, 0)))
        || Some(&'L') == tiles.get(&(start + Coordinate::new(-1, 0)))
    {
        // left
        start + Coordinate::new(-1, 0)
    } else {
        panic!("Starting point is not properly connected to another pipe!");
    }
}

/// move ahead through the pipes
fn move_ahead(current: char, direction: Coordinate) -> Coordinate {
    match (current, direction) {
        ('|', Coordinate { x: 0, y: 1 }) => Coordinate::new(0, 1),
        ('|', Coordinate { x: 0, y: -1 }) => Coordinate::new(0, -1),
        ('-', Coordinate { x: 1, y: 0 }) => Coordinate::new(1, 0),
        ('-', Coordinate { x: -1, y: 0 }) => Coordinate::new(-1, 0),
        ('F', Coordinate { x: 0, y: -1 }) => Coordinate::new(1, 0),
        ('F', Coordinate { x: -1, y: 0 }) => Coordinate::new(0, 1),
        ('L', Coordinate { x: 0, y: 1 }) => Coordinate::new(1, 0),
        ('L', Coordinate { x: -1, y: 0 }) => Coordinate::new(0, -1),
        ('J', Coordinate { x: 0, y: 1 }) => Coordinate::new(-1, 0),
        ('J', Coordinate { x: 1, y: 0 }) => Coordinate::new(0, -1),
        ('7', Coordinate { x: 0, y: -1 }) => Coordinate::new(-1, 0),
        ('7', Coordinate { x: 1, y: 0 }) => Coordinate::new(0, 1),
        _ => panic!("Invalid move"),
    }
}

fn part_1(input: &str) -> u32 {
    // initialise grid of tiles
    let tiles = Tiles::new(input);

    // find start of loop
    let start = tiles.find_value(&'S').unwrap();

    // find first move
    let mut previous = start;
    let mut current = find_first_move(&tiles, start);

    // loop until we end up at the start again
    let mut loop_length = 1;
    loop {
        if tiles[current] == 'S' {
            break;
        }

        // keep track of which direction we're heading
        let direction = current - previous;
        previous = current;

        // go to new tile
        current += move_ahead(tiles[current], direction);

        // update loop length
        loop_length += 1;
    }

    // furthest distance away from start is halfway the loop
    loop_length / 2
}

fn part_2(input: &str) -> isize {
    // initialise grid of tiles
    let tiles = Tiles::new(input);

    // find start of loop
    let start = tiles.find_value(&'S').unwrap();

    // find first move
    let mut current = find_first_move(&tiles, start);
    let mut previous = start;

    // construct loop
    let mut loop_main = [start, current].into_iter().collect::<Vec<Coordinate>>();
    loop {
        if tiles[current] == 'S' {
            break;
        }

        // keep track of which direction we're heading
        let direction = current - previous;
        previous = current;

        // go to new tile
        current += move_ahead(tiles[current], direction);

        // append current position to loop
        // loop_main.insert(current);
        loop_main.push(current);
    }

    // find tiles enclosed in the main loop by using the shoelace formula and Pick's theorem
    // shoelace formula: area of simple polygon whose vertices are in the same xy-plane
    // A = sum_(i=1..=n) ( x_i * y_(i+1) - x_(i+1) * y_i )
    let loop_length = loop_main.len();
    let area = (1..=loop_length)
        .map(|i| {
            loop_main[i % loop_length].x * loop_main[(i + 1) % loop_length].y
                - loop_main[(i + 1) % loop_length].x * loop_main[i % loop_length].y
        })
        .sum::<isize>()
        / 2;

    // Pick's theorem: area of a simple polygon with integer vertex coordinates
    // A = i + b / 2 - 1 => i = A - b / 2 + 1
    // A = area -> determined using shoelace formula
    // i = interior points -> this is what we want!!
    // b = boundary points -> number of points in the loop
    area.abs() - loop_length as isize / 2 + 1
    // 0
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", part_1(&inp));
    println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(
            super::part_1(
                r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            4
        );

        assert_eq!(
            super::part_1(
                r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            8
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            super::part_2(
                r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            1
        );

        assert_eq!(
            super::part_2(
                r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );

        assert_eq!(
            super::part_2(
                r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
    }
}
