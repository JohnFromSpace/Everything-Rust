use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct XFastTrie {
    root: Node,
    max_bits: usize,
}


#[derive(Debug, Clone)]
enum Node {
    Leaf(BTreeSet<usize>),
    Internal(Box<[Option<Box<Node>>; 2]>),
}

impl XFastTrie {
    fn new() -> Self {
        XFastTrie {
            root: Node::Leaf(BTreeSet::new()),
            max_bits: 0,
        }
    }
    
    fn insert(&mut self, x: usize) {
        self.max_bits = usize::max(self.max_bits, x.count_ones() as usize);

        let mut current = &mut self.root;
        for i in (0..self.max_bits).rev() {
            let bit = (x >> i) & 1;
            current = match current {
                Node::Leaf(set) => {
                    let mut new_set = BTreeSet::new();
                    new_set.insert(x);
                    let mut internal_node = [None, None];
                    internal_node[bit] = Some(Box::new(Node::Leaf(new_set)));
                    Some(Box::new(Node::Internal(Box::new(internal_node))))
                }
                Node::Internal(ref mut internal_node) => {
                    if let Some(child) = &mut internal_node[bit] {
                        child.clone()
                    } else {
                        let mut new_set = BTreeSet::new();
                        new_set.insert(x);
                        internal_node[bit] = Some(Box::new(Node::Leaf(new_set)));
                        return;
                    }
                }
            };
        }
    }
    
    fn contains(&self, x: usize) -> bool {
        let mut current = &self.root;
        for i in (0..self.max_bits).rev() {
            let bit = (x >> i) & 1;
            current = match current {
                Node::Leaf(set) => set.contains(&x),
                Node::Internal(internal_node) => {
                    if let Some(child) = &internal_node[bit] {
                        child.clone()
                    } else {
                        return false;
                    }
                }
            };
        }
        true
    }
}
