use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}
