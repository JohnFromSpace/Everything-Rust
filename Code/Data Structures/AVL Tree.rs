use std::cmp::Ordering;

#[derive(Debug)]
struct AVLTree<T> {
    root: Option<Box<AVLNode<T>>>,
}

#[derive(Debug)]
struct AVLNode<T> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let new_root = self.insert_recursive(self.root.take(), value);
        self.root = new_root;
    }

    fn insert_recursive(&mut self, node: Option<Box<AVLNode<T>>>, value: T) -> Option<Box<AVLNode<T>>> {
        match node {
            None => Some(Box::new(AVLNode {
                value,
                height: 1,
                left: None,
                right: None,
            })),
            Some(mut current) => {
                match value.cmp(&current.value) {
                    Ordering::Less => {
                        current.left = self.insert_recursive(current.left.take(), value);
                    }
                    Ordering::Greater => {
                        current.right = self.insert_recursive(current.right.take(), value);
                    }
                    Ordering::Equal => {
                        // Duplicate values are not allowed in this example.
                        return Some(current);
                    }
                }

                current.height = 1 + current.left.as_ref().map_or(0, |n| n.height).max(
                    current.right.as_ref().map_or(0, |n| n.height),
                );

                let balance = self.get_balance(&current);

                // Left Heavy
                if balance > 1 {
                    if value < current.left.as_ref().unwrap().value {
                        // Left-Left case
                        return Some(self.rotate_right(current));
                    } else {
                        // Left-Right case
                        current.left = Some(self.rotate_left(current.left.take().unwrap()));
                        return Some(self.rotate_right(current));
                    }
                }
                // Right Heavy
                if balance < -1 {
                    if value > current.right.as_ref().unwrap().value {
                        // Right-Right case
                        return Some(self.rotate_left(current));
                    } else {
                        // Right-Left case
                        current.right = Some(self.rotate_right(current.right.take().unwrap()));
                        return Some(self.rotate_left(current));
                    }
                }

                Some(current)
            }
        }
    }

    fn get_balance(&self, node: &AVLNode<T>) -> isize {
        let left_height = node.left.as_ref().map_or(0, |n| n.height as isize);
        let right_height = node.right.as_ref().map_or(0, |n| n.height as isize);

        left_height - right_height
    }

    fn rotate_left(&mut self, mut node: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut new_root = node.right.take().unwrap();
        let subtree = new_root.left.take();

        node.right = subtree;
        new_root.left = Some(node);

        // Update heights
        node.height = 1 + node.left.as_ref().map_or(0, |n| n.height).max(
            node.right.as_ref().map_or(0, |n| n.height),
        );
        new_root.height = 1 + new_root.left.as_ref().map_or(0, |n| n.height).max(
            new_root.right.as_ref().map_or(0, |n| n.height),
        );

        new_root
    }

    fn rotate_right(&mut self, mut node: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut new_root = node.left.take().unwrap();
        let subtree = new_root.right.take();

        node.left = subtree;
        new_root.right = Some(node);

        // Update heights
        node.height = 1 + node.left.as_ref().map_or(0, |n| n.height).max(
            node.right.as_ref().map_or(0, |n| n.height),
        );
        new_root.height = 1 + new_root.left.as_ref().map_or(0, |n| n.height).max(
            new_root.right.as_ref().map_or(0, |n| n.height),
        );

        new_root
    }
}

fn main() {
    let mut avl_tree = AVLTree::new();

    avl_tree.insert(5);
    avl_tree.insert(3);
    avl_tree.insert(7);
    avl_tree.insert(2);
    avl_tree.insert(4);
    avl_tree.insert(6);
    avl_tree.insert(8);

    println!("{:?}", avl_tree);
}
