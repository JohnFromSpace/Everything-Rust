use std::collections::HashMap;

#[derive(Debug)]
struct PalindromicTree {
    nodes: Vec<PalindromicNode>,
    text: Vec<char>,
    last_added: usize,
}

#[derive(Debug)]
struct PalindromicNode {
    length: usize,
    suffix_link: usize,
    next: HashMap<char, usize>,
}

impl PalindromicTree {
    fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(PalindromicNode::new(0));
        nodes.push(PalindromicNode::new(-1));
        PalindromicTree {
            nodes,
            text: Vec::new(),
            last_added: 1,
        }
    }

    fn add_char(&mut self, c: char) {
        self.text.push(c);
        let mut current = self.last_added;

        while !self.is_palindrome(current, self.text.len() - 1) {
            current = self.nodes[current].suffix_link;
        }

        if let Some(&next_node) = self.nodes[current].next.get(&c) {
            self.last_added = next_node;
            return;
        }

        let new_node = self.nodes.len();
        self.nodes.push(PalindromicNode::new(self.nodes[current].length + 2));
        self.nodes[current].next.insert(c, new_node);

        if self.nodes[new_node].length == 1 {
            self.nodes[new_node].suffix_link = 1;
            self.last_added = new_node;
            return;
        }

        current = self.nodes[current].suffix_link;

        while !self.is_palindrome(current, self.text.len() - 1) {
            current = self.nodes[current].suffix_link;
        }

        self.nodes[new_node].suffix_link = match self.nodes[current].next.get(&c) {
            Some(&next_node) => next_node,
            None => 1,
        };

        self.last_added = new_node;
    }

    fn is_palindrome(&self, node: usize, end: usize) -> bool {
        let start = end - self.nodes[node].length + 1;
        start > 0 && self.text[start - 1] == self.text[end]
    }
}

impl PalindromicNode {
    fn new(length: isize) -> Self {
        PalindromicNode {
            length: length as usize,
            suffix_link: 0,
            next: HashMap::new(),
        }
    }
}

fn main() {
    let mut pal_tree = PalindromicTree::new();
    let input = "abacaba";

    for c in input.chars() {
        pal_tree.add_char(c);
    }

    println!("{:?}", pal_tree);
}
