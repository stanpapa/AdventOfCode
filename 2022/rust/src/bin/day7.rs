use std::{error::Error, fmt};

use aoc_lib::{
    graph::graph::{Graph, Vertex},
    io::input::Input,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Item {
    Dir(String),
    File(String),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Dir(name) => name,
            Item::File(name) => name,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

type FileSystem = Graph<Item>;

/// calculate size of current directory by summing of the sizes
/// of all files and directories contained in this directory
fn calc_dir_weight(fs: &FileSystem, dir: &Vertex<Item>) -> usize {
    match fs.get(dir) {
        Some(values) => values
            .iter()
            .map(|v| match v.get() {
                Item::Dir(_) => calc_dir_weight(fs, v),
                Item::File(_) => v.weight(),
            })
            .sum(),
        None => 0,
    }
}

/// Build the file system ( = graph ) from input
fn construct_file_system(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let mut cwd = Item::Dir("/".to_string());

    for l in input.lines().skip(1) {
        let tokens = l.split(' ').collect::<Vec<&str>>();

        // parse commands
        if tokens[0] == "$" {
            // `ls` can be skipped
            if tokens[1] != "cd" {
                continue;
            }

            // only cd commands at this point
            if tokens[2] == ".." {
                cwd = fs
                    .find_parent(&Vertex::new(cwd.clone(), 0))
                    .unwrap_or_else(|| panic!("Couldn't find parent!"))
                    .get_owned();
                continue;
            }

            // use full path name to prevent duplicate keys
            let cwd_new = cwd.name().to_string() + tokens[2] + "/";
            cwd = Item::Dir(cwd_new);
            continue;
        }

        // fill graph
        match tokens[0] {
            // add directory
            "dir" => {
                let dir_new = cwd.name().to_string() + tokens[1] + "/";
                fs.add_vertex_to(
                    Vertex::new(cwd.clone(), 0),
                    Vertex::new(Item::Dir(dir_new), 0),
                );
            }
            // add file
            _ => fs.add_vertex_to(
                Vertex::new(cwd.clone(), 0),
                Vertex::new(
                    Item::File(tokens[1].to_string()),
                    tokens[0].parse().unwrap(),
                ),
            ),
        };
    }

    fs
}

fn solve(input: &str, part_1: bool) -> usize {
    let fs = construct_file_system(input);

    match part_1 {
        true => fs
            .keys()
            .map(|key| calc_dir_weight(&fs, key))
            .filter(|val| val <= &100000)
            .sum(),
        false => {
            let total = calc_dir_weight(&fs, &Vertex::new(Item::Dir("/".to_string()), 0));
            *fs.keys()
                .map(|key| calc_dir_weight(&fs, key))
                .filter(|val| (70000000 - total + val) >= 30000000)
                .collect::<Vec<usize>>()
                .iter()
                .min()
                .unwrap()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = Input::new();

    println!("part 1: {:?}", solve(&inp.to_string(), true));
    println!("part 2: {:?}", solve(&inp.to_string(), false));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_7_part_1() {
        assert_eq!(
            super::solve(
                r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
                true
            ),
            95437
        );
    }

    #[test]
    fn day_7_part_2() {
        assert_eq!(
            super::solve(
                r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
                false
            ),
            24933642
        );
    }
}
