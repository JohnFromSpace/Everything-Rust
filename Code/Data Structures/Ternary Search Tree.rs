#[derive(Debug)]
struct Node {
    character: char,
    is_end_of_word: bool,
    left: Option<Box<Node>>,
    middle: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(character: char) -> Self {
        Node {
            character,
            is_end_of_word: false,
            left: None,
            middle: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct TernarySearchTree {
    root: Option<Box<Node>>,
}

impl TernarySearchTree {
    pub fn new() -> Self {
        TernarySearchTree { root: None }
    }

    pub fn insert(&mut self, key: &str) {
        if key.is_empty() {
            return;
        }

        self.root = Self::insert_recursive(self.root.take(), key.chars().peekable());
    }

    fn insert_recursive(
        node: Option<Box<Node>>,
        mut key_chars: std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<Node>> {
        if let Some(mut current) = node {
            let current_char = current.character;

            match key_chars.peek() {
                Some(&key_char) if key_char < current_char => {
                    current.left = Self::insert_recursive(current.left.take(), key_chars);
                }
                Some(&key_char) if key_char == current_char => {
                    key_chars.next(); // Consume the character
                    if key_chars.peek().is_none() {
                        current.is_end_of_word = true;
                    } else {
                        current.middle = Self::insert_recursive(current.middle.take(), key_chars);
                    }
                }
                _ => {
                    current.right = Self::insert_recursive(current.right.take(), key_chars);
                }
            }

            Some(current)
        } else {
            if let Some(&key_char) = key_chars.peek() {
                let mut new_node = Box::new(Node::new(key_char));
                key_chars.next(); // Consume the character

                if key_chars.peek().is_none() {
                    new_node.is_end_of_word = true;
                } else {
                    new_node.middle = Self::insert_recursive(new_node.middle.take(), key_chars);
                }

                Some(new_node)
            } else {
                None
            }
        }
    }

    pub fn search(&self, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }

        Self::search_recursive(self.root.as_ref(), key.chars().peekable())
    }

    fn search_recursive(
        node: Option<&Box<Node>>,
        mut key_chars: std::iter::Peekable<std::str::Chars>,
    ) -> bool {
        if let Some(current) = node {
            let current_char = current.character;

            match key_chars.peek() {
                Some(&key_char) if key_char < current_char => {
                    Self::search_recursive(current.left.as_ref(), key_chars)
                }
                Some(&key_char) if key_char == current_char => {
                    key_chars.next(); // Consume the character
                    if key_chars.peek().is_none() {
                        current.is_end_of_word
                    } else {
                        Self::search_recursive(current.middle.as_ref(), key_chars)
                    }
                }
                _ => Self::search_recursive(current.right.as_ref(), key_chars),
            }
        } else {
            false
        }
    }
}

fn main() {
    let mut tst = TernarySearchTree::new();

    tst.insert("apple");
    tst.insert("banana");
    tst.insert("orange");

    println!("Search 'apple': {}", tst.search("apple"));
    println!("Search 'grape': {}", tst.search("grape"));
}

