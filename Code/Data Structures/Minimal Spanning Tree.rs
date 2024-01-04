use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vertex(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    from: Vertex,
    to: Vertex,
    weight: usize,
}

#[derive(Debug)]
struct Graph {
    vertices: HashSet<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: HashSet::new(),
            edges: Vec::new(),
        }
    }

    fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex);
    }

    fn add_edge(&mut self, from: Vertex, to: Vertex, weight: usize) {
        self.edges.push(Edge { from, to, weight });
    }

    fn kruskal_mst(&self) -> Vec<Edge> {
        let mut mst_edges = Vec::new();
        let mut disjoint_sets = DisjointSets::new();

        // Initialize disjoint sets with each vertex in its own set
        for vertex in &self.vertices {
            disjoint_sets.make_set(vertex.clone());
        }

        // Sort edges by weight in ascending order
        let mut sorted_edges = BinaryHeap::new();
        for edge in &self.edges {
            sorted_edges.push(ReverseEdge(edge.clone()));
        }

        while let Some(ReverseEdge(edge)) = sorted_edges.pop() {
            if disjoint_sets.find_set(&edge.from) != disjoint_sets.find_set(&edge.to) {
                mst_edges.push(edge.clone());
                disjoint_sets.union_sets(edge.from.clone(), edge.to.clone());
            }
        }

        mst_edges
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DisjointSetElement<T> {
    value: T,
    rank: usize,
    parent: T,
}

#[derive(Debug)]
struct DisjointSets<T: PartialEq + Eq + Hash + Clone> {
    elements: HashMap<T, DisjointSetElement<T>>,
}

impl<T: PartialEq + Eq + Hash + Clone> DisjointSets<T> {
    fn new() -> Self {
        DisjointSets {
            elements: HashMap::new(),
        }
    }

    fn make_set(&mut self, value: T) {
        if !self.elements.contains_key(&value) {
            let element = DisjointSetElement {
                value: value.clone(),
                rank: 0,
                parent: value.clone(),
            };
            self.elements.insert(value, element);
        }
    }

    fn find_set(&mut self, value: T) -> T {
        if let Some(element) = self.elements.get(&value).cloned() {
            if element.value != element.parent {
                let root = self.find_set(element.parent.clone());
                self.elements.get_mut(&value).unwrap().parent = root.clone();
                root
            } else {
                element.value
            }
        } else {
            panic!("Element not found in the disjoint sets.");
        }
    }

    fn union_sets(&mut self, value1: T, value2: T) {
        let root1 = self.find_set(value1.clone());
        let root2 = self.find_set(value2.clone());

        if let (Some(element1), Some(element2)) = (self.elements.get_mut(&root1), self.elements.get_mut(&root2)) {
            if root1 != root2 {
                if element1.rank < element2.rank {
                    element1.parent = root2.clone();
                } else if element1.rank > element2.rank {
                    element2.parent = root1.clone();
                } else {
                    element1.parent = root2.clone();
                    element2.rank += 1;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ReverseEdge(Edge);

impl PartialOrd for ReverseEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.weight.cmp(&self.0.weight))
    }
}

impl Ord for ReverseEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.weight.cmp(&self.0.weight)
    }
}

fn main() {
    let mut graph = Graph::new();

    let v1 = Vertex(1);
    let v2 = Vertex(2);
    let v3 = Vertex(3);
    let v4 = Vertex(4);

    graph.add_vertex(v1.clone());
    graph.add_vertex(v2.clone());
    graph.add_vertex(v3.clone());
    graph.add_vertex(v4.clone());

    graph.add_edge(v1.clone(), v2.clone(), 1);
    graph.add_edge(v1.clone(), v3.clone(), 2);
    graph.add_edge(v2.clone(), v3.clone(), 3);
    graph.add_edge(v2.clone(), v4.clone(), 4);
    graph.add_edge(v3.clone(), v4.clone(), 5);

    let mst_edges = graph.kruskal_mst();

    println!("Minimal Spanning Tree Edges: {:?}", mst_edges);
}
