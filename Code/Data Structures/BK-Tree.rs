use std::collections::HashMap;

#[derive(Debug)]
struct BKNode<T: PartialEq + Eq> {
    value: T,
    children: HashMap<usize, Box<BKNode<T>>>,
}

impl<T: PartialEq + Eq> BKNode<T> {
    fn new(value: T) -> Self {
        BKNode {
            value,
            children: HashMap::new(),
        }
    }

    fn add_child(&mut self, distance: usize, child: BKNode<T>) {
        self.children.insert(distance, Box::new(child));
    }
}

#[derive(Debug)]
pub struct BKTree<T: PartialEq + Eq> {
    root: Option<BKNode<T>>,
}

impl<T: PartialEq + Eq> BKTree<T> {
    pub fn new() -> Self {
        BKTree { root: None }
    }

    pub fn add(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            self.add_recursive(root, value);
        } else {
            self.root = Some(BKNode::new(value));
        }
    }

    fn add_recursive(&mut self, node: &mut BKNode<T>, value: T) {
        let distance = // Calculate the distance between `node.value` and `value` (use your own metric);
        if let Some(child) = node.children.get_mut(&distance) {
            self.add_recursive(child, value);
        } else {
            node.add_child(distance, BKNode::new(value));
        }
    }

    pub fn search(&self, target: T, threshold: usize) -> Vec<&T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            self.search_recursive(root, &target, threshold, &mut result);
        }
        result
    }

    fn search_recursive(&self, node: &BKNode<T>, target: &T, threshold: usize, result: &mut Vec<&T>) {
        let distance = // Calculate the distance between `node.value` and `target` (use your own metric);
        if distance <= threshold {
            result.push(&node.value);
        }

        let min_distance = distance.saturating_sub(threshold);
        let max_distance = distance + threshold;

        for (&child_distance, child) in &node.children {
            if child_distance >= min_distance && child_distance <= max_distance {
                self.search_recursive(child, target, threshold, result);
            }
        }
    }
}

fn main() {
    let mut bk_tree = BKTree::new();

    bk_tree.add("book");
    bk_tree.add("back");
    bk_tree.add("bat");
    bk_tree.add("bark");
    bk_tree.add("cat");

    let search_result = bk_tree.search("back", 1);
    println!("Similar words to 'back': {:?}", search_result);
}
