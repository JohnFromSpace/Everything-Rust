use std::cmp::Ord;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(self.insert_recursive(self.root.take(), value));
    }

    fn insert_recursive(&mut self, node: Option<Box<Node<T>>>, value: T) -> Box<Node<T>> {
        match node {
            None => Box::new(Node::new(value)),
            Some(mut current) => {
                if value <= current.value {
                    current.left = Some(self.insert_recursive(current.left.take(), value));
                } else {
                    current.right = Some(self.insert_recursive(current.right.take(), value));
                }
                current
            }
        }
    }

    pub fn contains(&self, value: T) -> bool {
        self.contains_recursive(&self.root, value)
    }

    fn contains_recursive(&self, node: &Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(current) => {
                if value == current.value {
                    true
                } else if value < current.value {
                    self.contains_recursive(&current.left, value)
                } else {
                    self.contains_recursive(&current.right, value)
                }
            }
        }
    }

    pub fn inorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.inorder_traversal_recursive(&self.root, &mut result);
        result
    }

    fn inorder_traversal_recursive(&self, node: &Option<Box<Node<T>>>, result: &mut Vec<&T>) {
        if let Some(current) = node {
            self.inorder_traversal_recursive(&current.left, result);
            result.push(&current.value);
            self.inorder_traversal_recursive(&current.right, result);
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(2);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    println!("Inorder Traversal: {:?}", bst.inorder_traversal());

    println!("Contains 4: {}", bst.contains(4)); // Output: true
    println!("Contains 9: {}", bst.contains(9)); // Output: false
}
