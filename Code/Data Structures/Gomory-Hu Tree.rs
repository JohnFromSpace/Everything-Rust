use std::collections::{HashMap, HashSet};
use std::cmp::min;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    from: usize,
    to: usize,
    capacity: usize,
}

#[derive(Debug)]
struct GomoryHuTree {
    graph: Vec<Vec<Edge>>,
}

impl GomoryHuTree {
    fn new(num_nodes: usize) -> Self {
        GomoryHuTree {
            graph: vec![vec![]; num_nodes],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: usize) {
        let edge = Edge { from, to, capacity };
        self.graph[from].push(edge.clone());
        self.graph[to].push(Edge {
            from: to,
            to: from,
            capacity,
        });
    }

    fn gomory_hu_tree(&self) -> GomoryHuTree {
        let n = self.graph.len();
        let mut tree = GomoryHuTree::new(n);

        for i in 1..n {
            let mut parent = vec![0; n];
            let mut min_capacity = vec![usize::MAX; n];

            for j in 0..i {
                let cut_capacity = self.min_cut(i, j);
                if cut_capacity < min_capacity[i] {
                    min_capacity[i] = cut_capacity;
                    parent[i] = j;
                }
            }

            tree.add_edge(i, parent[i], min_capacity[i]);
        }

        tree
    }

    fn min_cut(&self, s: usize, t: usize) -> usize {
        let mut residual_graph = self.graph.clone();

        let mut parent = vec![0; self.graph.len()];
        let mut flow = 0;

        while self.find_augmenting_path(s, t, &residual_graph, &mut parent) {
            let path_capacity = self.calculate_path_capacity(s, t, &residual_graph, &parent);
            flow += path_capacity;

            self.update_residual_graph(s, t, &mut residual_graph, &parent, path_capacity);
        }

        flow
    }

    fn find_augmenting_path(
        &self,
        s: usize,
        t: usize,
        residual_graph: &Vec<Vec<Edge>>,
        parent: &mut Vec<usize>,
    ) -> bool {
        let mut visited = vec![false; self.graph.len()];
        let mut queue = Vec::new();

        visited[s] = true;
        queue.push(s);

        while let Some(current) = queue.pop() {
            for edge in &residual_graph[current] {
                if !visited[edge.to] && edge.capacity > 0 {
                    visited[edge.to] = true;
                    parent[edge.to] = current;

                    if edge.to == t {
                        return true;
                    }

                    queue.push(edge.to);
                }
            }
        }

        false
    }

    fn calculate_path_capacity(
        &self,
        s: usize,
        t: usize,
        residual_graph: &Vec<Vec<Edge>>,
        parent: &Vec<usize>,
    ) -> usize {
        let mut capacity = usize::MAX;
        let mut current = t;

        while current != s {
            let edge = &residual_graph[parent[current]]
                .iter()
                .find(|e| e.to == current)
                .unwrap();
            capacity = min(capacity, edge.capacity);
            current = parent[current];
        }

        capacity
    }

    fn update_residual_graph(
        &self,
        s: usize,
        t: usize,
        residual_graph: &mut Vec<Vec<Edge>>,
        parent: &Vec<usize>,
        path_capacity: usize,
    ) {
        let mut current = t;

        while current != s {
            let edge_index = residual_graph[parent[current]]
                .iter()
                .position(|e| e.to == current)
                .unwrap();

            residual_graph[parent[current]][edge_index].capacity -= path_capacity;

            let reverse_edge_index = residual_graph[current]
                .iter()
                .position(|e| e.to == parent[current])
                .unwrap();

            residual_graph[current][reverse_edge_index].capacity += path_capacity;

            current = parent[current];
        }
    }
}

fn main() {
    let mut graph = GomoryHuTree::new(5);

    graph.add_edge(0, 1, 3);
    graph.add_edge(0, 2, 5);
    graph.add_edge(1, 2, 1);
    graph.add_edge(1, 3, 7);
    graph.add_edge(2, 3, 2);
    graph.add_edge(2, 4, 4);
    graph.add_edge(3, 4, 6);

    let gomory_hu_tree = graph.gomory_hu_tree();

    println!("Gomory-Hu Tree: {:?}", gomory_hu_tree.graph);
}
