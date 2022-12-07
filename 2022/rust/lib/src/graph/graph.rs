use std::{
    collections::{hash_map::Keys, HashMap},
    fmt,
    hash::Hash,
};

pub trait GraphTraits: PartialEq + Eq + Hash + fmt::Debug + fmt::Display + Clone {}

impl<T: PartialEq + Eq + Hash + fmt::Debug + fmt::Display + Clone> GraphTraits for T {}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub struct Vertex<T: GraphTraits> {
    v: T,
    weight: usize,
}

impl<T: GraphTraits> Vertex<T> {
    pub fn new(v: T, weight: usize) -> Self {
        Self { v, weight }
    }

    pub fn get(&self) -> &T {
        &self.v
    }

    pub fn get_owned(&self) -> T {
        self.v.clone()
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}

/// This breaks down, once we have duplicate keys
#[derive(PartialEq, Debug)]
pub struct Graph<T: GraphTraits>(HashMap<Vertex<T>, Vec<Vertex<T>>>);

// use adjacency list?

impl<T: GraphTraits> Graph<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_vertex(&mut self, vertex: Vertex<T>) {
        self.0.entry(vertex).or_insert(Vec::new());
    }

    pub fn add_vertex_to(&mut self, from: Vertex<T>, to: Vertex<T>) {
        self.0.entry(from).or_insert(Vec::new()).push(to);
    }

    pub fn find_parent(&self, vertex: &Vertex<T>) -> Option<Vertex<T>> {
        for (key, value) in self.iter() {
            for v in value.iter() {
                if v == vertex {
                    return Some(key.clone());
                }
            }
        }
        None
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Vertex<T>, &Vec<Vertex<T>>)> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<'_, Vertex<T>, Vec<Vertex<T>>> {
        self.0.keys()
    }

    pub fn get(&self, key: &Vertex<T>) -> Option<&Vec<Vertex<T>>> {
        self.0.get(key)
    }
}

impl<T: GraphTraits> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in self.0.iter() {
            writeln!(f, "{:?} -> {:?}", k, v)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, Vertex};
    use std::collections::HashMap;

    #[test]
    fn add_vertex_from() {
        let mut g = Graph::<u8>::new();
        g.add_vertex_to(Vertex::new(0, 0), Vertex::new(1, 0));
        g.add_vertex_to(Vertex::new(0, 0), Vertex::new(2, 0));
        g.add_vertex_to(Vertex::new(1, 0), Vertex::new(3, 0));
        assert_eq!(
            Graph::<u8>(HashMap::from([
                (
                    Vertex::new(0, 0),
                    vec![Vertex::new(1, 0), Vertex::new(2, 0)],
                ),
                (Vertex::new(1, 0), vec![Vertex::new(3, 0)]),
            ])),
            g
        );
    }

    #[test]
    fn find_parent() {
        let g = Graph::<u8>(HashMap::from([
            (
                Vertex::new(0, 0),
                vec![Vertex::new(1, 0), Vertex::new(2, 0)],
            ),
            (Vertex::new(1, 0), vec![Vertex::new(3, 0)]),
        ]));
        assert_eq!(g.find_parent(&Vertex::new(2, 0)), Some(Vertex::new(0, 0)));
        assert_eq!(g.find_parent(&Vertex::new(2, 1)), None);
        assert_eq!(g.find_parent(&Vertex::new(4, 0)), None);
    }
}
