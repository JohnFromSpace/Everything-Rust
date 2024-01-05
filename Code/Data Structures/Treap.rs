use rand::Rng;

#[derive(Debug)]
struct TreapNode<K: Ord> {
    key: K,
    priority: i32,
    left: Option<Box<TreapNode<K>>>,
    right: Option<Box<TreapNode<K>>>,
}

impl<K: Ord> TreapNode<K> {
    fn new(key: K, priority: i32) -> Self {
        TreapNode {
            key,
            priority,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct Treap<K: Ord> {
    root: Option<Box<TreapNode<K>>>,
}

impl<K: Ord> Treap<K> {
    pub fn new() -> Self {
        Treap { root: None }
    }

    fn rotate_right(mut node: Box<TreapNode<K>>) -> Box<TreapNode<K>> {
        if let Some(mut left) = node.left.take() {
            node.left = left.right.take();
            left.right = Some(node);
            return left;
        }
        node
    }

    fn rotate_left(mut node: Box<TreapNode<K>>) -> Box<TreapNode<K>> {
        if let Some(mut right) = node.right.take() {
            node.right = right.left.take();
            right.left = Some(node);
            return right;
        }
        node
    }

    fn insert_recursive(
        node: Option<Box<TreapNode<K>>>,
        key: K,
        priority: i32,
    ) -> Box<TreapNode<K>> {
        match node {
            Some(mut current) => {
                if key < current.key {
                    if priority > current.priority {
                        current.left = Some(Self::insert_recursive(current.left.take(), key, priority));
                        return Self::rotate_right(current);
                    } else {
                        current.left = Some(Self::insert_recursive(current.left.take(), key, priority));
                    }
                } else {
                    if priority > current.priority {
                        current.right = Some(Self::insert_recursive(current.right.take(), key, priority));
                        return Self::rotate_left(current);
                    } else {
                        current.right = Some(Self::insert_recursive(current.right.take(), key, priority));
                    }
                }
                current
            }
            None => Box::new(TreapNode::new(key, priority)),
        }
    }

    pub fn insert(&mut self, key: K) {
        let priority = rand::thread_rng().gen_range(0..1000); // Assign a random priority
        self.root = Some(Self::insert_recursive(self.root.take(), key, priority));
    }

    fn search_recursive(node: &Option<Box<TreapNode<K>>>, key: K) -> bool {
        match node {
            Some(current) => {
                if key == current.key {
                    true
                } else if key < current.key {
                    Self::search_recursive(&current.left, key)
                } else {
                    Self::search_recursive(&current.right, key)
                }
            }
            None => false,
        }
    }

    pub fn search(&self, key: K) -> bool {
        Self::search_recursive(&self.root, key)
    }
}

fn main() {
    let mut treap = Treap::new();

    treap.insert(3);
    treap.insert(7);
    treap.insert(1);
    treap.insert(5);

    println!("Search for key 5: {}", treap.search(5));
    println!("Search for key 2: {}", treap.search(2));
}
