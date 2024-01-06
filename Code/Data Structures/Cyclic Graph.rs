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

    fn detect_cycle(&self, node: usize, visited: &mut HashSet<usize>, recursion_stack: &mut HashSet<usize>) -> bool {
        visited.insert(node);
        recursion_stack.insert(node);

        if let Some(neighbors) = self.edges.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if self.detect_cycle(neighbor, visited, recursion_stack) {
                        return true;
                    }
                } else if recursion_stack.contains(&neighbor) {
                    return true; // Cycle detected
                }
            }
        }

        recursion_stack.remove(&node);
        false
    }
}
