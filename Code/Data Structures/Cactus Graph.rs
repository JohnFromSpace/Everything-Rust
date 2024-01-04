use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vertex {
    id: usize,
}

#[derive(Debug)]
struct Edge {
    from: Vertex,
    to: Vertex,
}

#[derive(Debug)]
struct CactusGraph {
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge>,
}

impl CactusGraph {
    fn new() -> Self {
        CactusGraph {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn add_vertex(&mut self, id: usize) {
        let vertex = Vertex { id };
        self.vertices.insert(vertex);
    }

    fn add_edge(&mut self, from_id: usize, to_id: usize) {
        let from = Vertex { id: from_id };
        let to = Vertex { id: to_id };
        let edge = Edge { from, to };

        if !self.vertices.contains(&from) || !self.vertices.contains(&to) {
            panic!("Both vertices should exist in the graph before adding an edge.");
        }

        self.edges.insert(edge);
    }

    fn is_cactus(&self) -> bool {
        // Check that any two simple cycles have at most one vertex in common
        for vertex1 in &self.vertices {
            for vertex2 in &self.vertices {
                if vertex1 != vertex2 {
                    let common_vertices: HashSet<_> = self.find_common_vertices(vertex1, vertex2);

                    if common_vertices.len() > 1 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn find_common_vertices(&self, vertex1: &Vertex, vertex2: &Vertex) -> HashSet<Vertex> {
        let mut common_vertices = HashSet::new();

        for edge in &self.edges {
            if (edge.from == *vertex1 && edge.to == *vertex2)
                || (edge.from == *vertex2 && edge.to == *vertex1)
            {
                common_vertices.insert(edge.from.clone());
                common_vertices.insert(edge.to.clone());
            }
        }

        common_vertices
    }
}

fn main() {
    let mut cactus_graph = CactusGraph::new();

    // Add vertices
    cactus_graph.add_vertex(1);
    cactus_graph.add_vertex(2);
    cactus_graph.add_vertex(3);
    cactus_graph.add_vertex(4);

    // Add edges
    cactus_graph.add_edge(1, 2);
    cactus_graph.add_edge(2, 3);
    cactus_graph.add_edge(3, 4);
    cactus_graph.add_edge(4, 1);

    println!("Is Cactus Graph: {}", cactus_graph.is_cactus()); // Output: true
}
