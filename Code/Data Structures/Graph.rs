use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph<T> {
    vertices: HashSet<T>,
    edges: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        Graph {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: T) {
        self.vertices.insert(vertex);
        self.edges.entry(vertex).or_insert(HashSet::new());
    }

    fn add_edge(&mut self, from: T, to: T) {
        self.vertices.insert(from.clone());
        self.vertices.insert(to.clone());

        let entry = self.edges.entry(from).or_insert(HashSet::new());
        entry.insert(to);
    }

    fn get_neighbors(&self, vertex: &T) -> Option<&HashSet<T>> {
        self.edges.get(vertex)
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_vertex("A");
    graph.add_vertex("B");
    graph.add_vertex("C");

    graph.add_edge("A", "B");
    graph.add_edge("A", "C");
    graph.add_edge("B", "C");

    println!("{:?}", graph);
    println!("Neighbors of A: {:?}", graph.get_neighbors(&"A"));
}
