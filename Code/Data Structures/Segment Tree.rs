#[derive(Debug)]
struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; 2 * n];

        // Build the leaves of the tree
        for i in 0..n {
            tree[n + i] = nums[i];
        }

        // Build the rest of the tree by combining values
        for i in (1..n).rev() {
            tree[i] = tree[i * 2] + tree[i * 2 + 1];
        }

        SegmentTree { n, tree }
    }

    fn update(&mut self, index: usize, value: i32) {
        let mut i = index + self.n;
        self.tree[i] = value;

        while i > 1 {
            i /= 2;
            self.tree[i] = self.tree[i * 2] + self.tree[i * 2 + 1];
        }
    }

    fn query(&self, left: usize, right: usize) -> i32 {
        let mut left_idx = left + self.n;
        let mut right_idx = right + self.n;
        let mut result = 0;

        while left_idx <= right_idx {
            if left_idx % 2 == 1 {
                result += self.tree[left_idx];
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                result += self.tree[right_idx];
                right_idx -= 1;
            }

            left_idx /= 2;
            right_idx /= 2;
        }

        result
    }
}

fn main() {
    let nums = vec![1, 3, 5, 7, 9, 11];
    let mut segment_tree = SegmentTree::new(nums.clone());

    println!("Original array: {:?}", nums);
    println!("Segment tree: {:?}", segment_tree);

    segment_tree.update(3, 8);

    println!("Updated array: {:?}", nums);
    println!("Updated segment tree: {:?}", segment_tree);

    let query_result = segment_tree.query(2, 5);
    println!("Query result for range [2, 5]: {}", query_result);
}
