use std::collections::HashSet;

#[derive(Debug)]
struct SparseSet {
    set: HashSet<usize>,
}

impl SparseSet {
    fn new() -> Self {
        SparseSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, value: usize) {
        self.set.insert(value);
    }

    fn remove(&mut self, value: usize) {
        self.set.remove(&value);
    }

    fn contains(&self, value: usize) -> bool {
        self.set.contains(&value)
    }

    fn clear(&mut self) {
        self.set.clear();
    }

    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    fn size(&self) -> usize {
        self.set.len()
    }
}

fn main() {
    let mut sparse_set = SparseSet::new();

    // Insert elements into the sparse set
    sparse_set.insert(3);
    sparse_set.insert(7);
    sparse_set.insert(11);

    // Check if elements are present in the sparse set
    println!("Contains 7: {}", sparse_set.contains(7));
    println!("Contains 5: {}", sparse_set.contains(5));

    // Remove an element from the sparse set
    sparse_set.remove(7);

    // Check if the sparse set is empty
    println!("Is empty: {}", sparse_set.is_empty());

    // Print the size of the sparse set
    println!("Size: {}", sparse_set.size());

    // Clear the sparse set
    sparse_set.clear();
    println!("Is empty after clear: {}", sparse_set.is_empty());
}
