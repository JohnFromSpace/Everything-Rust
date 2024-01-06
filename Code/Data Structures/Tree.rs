// Define a basic BinaryTreeNode structure
#[derive(Debug)]
struct BinaryTreeNode<T> {
    data: T,
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    // Create a new binary tree node
    fn new(data: T) -> Self {
        BinaryTreeNode {
            data,
            left: None,
            right: None,
        }
    }
}

// Define the BinaryTree structure
#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTree<T> {
    // Create a new empty binary tree
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // Insert a new element into the binary tree
    fn insert(&mut self, data: T) {
        self.root = Some(Self::insert_recursive(self.root.take(), data));
    }

    // Recursive helper function for insertion
    fn insert_recursive(node: Option<Box<BinaryTreeNode<T>>>, data: T) -> Box<BinaryTreeNode<T>> {
        match node {
            Some(mut current) => {
                if data < current.data {
                    current.left = Some(Self::insert_recursive(current.left.take(), data));
                } else {
                    current.right = Some(Self::insert_recursive(current.right.take(), data));
                }
                current
            }
            None => Box::new(BinaryTreeNode::new(data)),
        }
    }

    // Print the binary tree using in-order traversal
    fn print_in_order(&self) {
        Self::print_in_order_recursive(&self.root);
    }

    // Recursive helper function for in-order traversal
    fn print_in_order_recursive(node: &Option<Box<BinaryTreeNode<T>>>) {
        if let Some(current) = node {
            Self::print_in_order_recursive(&current.left);
            println!("{}", current.data);
            Self::print_in_order_recursive(&current.right);
        }
    }
}

fn main() {
    // Create a new binary tree of integers
    let mut binary_tree = BinaryTree::new();

    // Insert elements into the binary tree
    binary_tree.insert(5);
    binary_tree.insert(3);
    binary_tree.insert(7);
    binary_tree.insert(2);
    binary_tree.insert(4);

    // Print the binary tree in-order
    binary_tree.print_in_order();
}

