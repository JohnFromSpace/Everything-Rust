use std::ptr;

// Node representing an element in the Skip List
#[derive(Debug)]
struct Node {
    value: i32,
    // Pointers to the next node in each level
    forward: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new(value: i32, level: usize) -> Self {
        Node {
            value,
            forward: vec![None; level + 1],
        }
    }
}

// Skip List data structure
#[derive(Debug)]
struct SkipList {
    max_level: usize,
    head: Option<Box<Node>>,
}

impl SkipList {
    fn new(max_level: usize) -> Self {
        SkipList {
            max_level,
            head: None,
        }
    }

    fn random_level(&self) -> usize {
        let mut level = 0;
        while level < self.max_level && rand::random::<bool>() {
            level += 1;
        }
        level
    }

    fn insert(&mut self, value: i32) {
        let mut update = vec![None; self.max_level + 1];
        let mut x = self.head.clone();

        for level in (0..=self.max_level).rev() {
            while let Some(ref mut current) = x {
                if let Some(ref next) = current.forward[level] {
                    if next.value < value {
                        x = Some(next.clone());
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            update[level] = x.clone();
        }

        let level = self.random_level();
        let new_node = Box::new(Node::new(value, level));

        for i in 0..=level {
            if let Some(ref mut current) = update[i] {
                new_node.forward[i] = current.forward[i].take();
                current.forward[i] = Some(new_node.clone());
            }
        }

        if self.head.is_none() || level > self.head.as_ref().unwrap().forward.len() - 1 {
            self.head = Some(new_node);
        }
    }

    fn search(&self, value: i32) -> Option<i32> {
        let mut x = self.head.clone();

        for level in (0..=self.max_level).rev() {
            while let Some(ref current) = x {
                if let Some(ref next) = current.forward[level] {
                    if next.value < value {
                        x = Some(next.clone());
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if let Some(ref node) = x {
            if node.value == value {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn main() {
    let mut skip_list = SkipList::new(4);

    skip_list.insert(3);
    skip_list.insert(6);
    skip_list.insert(7);
    skip_list.insert(9);
    skip_list.insert(12);
    skip_list.insert(19);
    skip_list.insert(17);
    skip_list.insert(26);
    skip_list.insert(21);
    skip_list.insert(25);

    println!("{:?}", skip_list);

    let search_result = skip_list.search(19);
    match search_result {
        Some(value) => println!("Found: {}", value),
        None => println!("Not found"),
    }
}
