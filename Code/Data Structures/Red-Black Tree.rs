use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Debug, Clone)]
pub struct RedBlackTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let new_root = self.insert_recursive(self.root.take(), key, value);
        if let Some(mut node) = new_root {
            node.color = Color::Black;
            self.root = Some(node);
        }
    }

    fn insert_recursive(
        &mut self,
        node: Option<Box<Node<K, V>>>,
        key: K,
        value: V,
    ) -> Option<Box<Node<K, V>>> {
        match node {
            None => Some(Box::new(Node {
                key,
                value,
                color: Color::Red,
                left: None,
                right: None,
            })),
            Some(mut current) => {
                if key < current.key {
                    current.left = self.insert_recursive(current.left.take(), key, value);
                } else if key > current.key {
                    current.right = self.insert_recursive(current.right.take(), key, value);
                } else {
                    // Key already exists, update the value
                    current.value = value;
                }

                if is_red(&current.right) && !is_red(&current.left) {
                    current = self.rotate_left(current);
                }
                if is_red(&current.left) && is_red(&current.left.as_ref().unwrap().left) {
                    current = self.rotate_right(current);
                }
                if is_red(&current.left) && is_red(&current.right) {
                    self.flip_colors(&mut current);
                }

                Some(current)
            }
        }
    }

    fn rotate_left(&mut self, mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.right.take().unwrap();
        h.right = x.left.take();
        x.left = Some(h);
        x.color = h.color;
        h.color = Color::Red;
        x
    }

    fn rotate_right(&mut self, mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.left.take().unwrap();
        h.left = x.right.take();
        x.right = Some(h);
        x.color = h.color;
        h.color = Color::Red;
        x
    }

    fn flip_colors(&mut self, h: &mut Box<Node<K, V>>) {
        h.color = Color::Red;
        h.left.as_mut().unwrap().color = Color::Black;
        h.right.as_mut().unwrap().color = Color::Black;
    }
}

fn is_red<K, V>(node: &Option<Box<Node<K, V>>>) -> bool {
    match node {
        Some(n) => n.color == Color::Red,
        None => false,
    }
}

fn main() {
    let mut tree = RedBlackTree::new();

    tree.insert(3, "Three");
    tree.insert(1, "One");
    tree.insert(4, "Four");
    tree.insert(2, "Two");

    println!("{:?}", tree);
}
