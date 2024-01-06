use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph { edges: HashMap::new() }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for &node in self.edges.keys() {
            if !visited.contains(&node) {
                if self.detect_cycle(node, &mut visited, &mut recursion_stack) {
                    return true;
                }
            }
        }

        false
    }
}
