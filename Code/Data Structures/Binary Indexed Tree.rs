#[derive(Debug)]
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree { tree: vec![0; size + 1] }
    }

    fn update(&mut self, index: usize, delta: i32) {
        let mut i = index as i32 + 1;
        while i < self.tree.len() as i32 {
            self.tree[i as usize] += delta;
            i += i & -i;
        }
    }

    fn query(&self, mut index: usize) -> i32 {
        let mut result = 0;
        index += 1;
        while index > 0 {
            result += self.tree[index];
            index -= (index as i32 & -index as i32) as usize;
        }
        result
    }

    fn range_query(&self, start: usize, end: usize) -> i32 {
        if start > 0 {
            self.query(end) - self.query(start - 1)
        } else {
            self.query(end)
        }
    }
}

fn main() {
    let mut fenwick_tree = FenwickTree::new(10);

    // Update individual elements
    fenwick_tree.update(2, 3);
    fenwick_tree.update(5, 2);
    fenwick_tree.update(7, 1);

    // Query individual elements
    println!("Query at index 2: {}", fenwick_tree.query(2));  // Output: 3
    println!("Query at index 5: {}", fenwick_tree.query(5));  // Output: 5
    println!("Query at index 7: {}", fenwick_tree.query(7));  // Output: 6

    // Range query
    println!("Range query [2, 5]: {}", fenwick_tree.range_query(2, 5));  // Output: 5
}
