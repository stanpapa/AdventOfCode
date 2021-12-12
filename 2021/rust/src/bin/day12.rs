use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use aoc_2021_rust::utils::input;

#[derive(Debug)]
struct Graph {
    vertices: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(s: String) -> Self {
        let mut vertices = HashMap::<String, Vec<String>>::new();

        for line in s.lines() {
            let (begin, end) = line.split_once("-").unwrap();
            vertices
                .entry(begin.to_string())
                .or_insert(Vec::new())
                .push(end.to_string());
            vertices
                .entry(end.to_string())
                .or_insert(Vec::new())
                .push(begin.to_string());
        }

        Self { vertices }
    }
}

fn traverse(graph: &Graph, vertex: &str, mut seen: HashSet<String>, mut seen_twice: bool) -> usize {
    // cannot visit small caves more than twice
    if seen.contains(vertex) {
        if seen_twice {
            return 0;
        } else {
            seen_twice = true;
        }
    }

    // found end of path
    if vertex == "end" {
        return 1;
    }

    // if vertex is small cave, then add to seen
    if vertex == vertex.to_lowercase() {
        seen.insert(vertex.to_string());
    }

    // continue traversing the graph
    graph.vertices[vertex]
        .iter()
        .filter(|&vertex| vertex != "start") // cannot visit start twice
        .map(|vertex| traverse(graph, vertex, seen.clone(), seen_twice))
        .sum()
}

// small caves can only be visited once
fn part_1(graph: &Graph) -> usize {
    traverse(graph, "start", HashSet::new(), true)
}

// small caves can be visited twice
// start can only be visited once
fn part_2(graph: &Graph) -> usize {
    traverse(graph, "start", HashSet::new(), false)
}

fn main() -> Result<(), Error> {
    let input = input::read_input_as_string()?;
    let graph = Graph::new(input);

    println!("Part 1: {}", part_1(&graph));
    println!("Part 2: {}", part_2(&graph));

    Ok(())
}
