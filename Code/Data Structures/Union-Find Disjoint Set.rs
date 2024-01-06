// Define the UnionFind structure
#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    // Create a new Union-Find data structure with 'n' elements
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        UnionFind { parent, rank }
    }

    // Find the representative (root) of the set to which 'x' belongs
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    // Merge the sets containing elements 'x' and 'y'
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by rank to keep the tree balanced
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
                self.rank[root_y] += 1;
            }
        }
    }
}

fn main() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(1, 4);

    // Check if elements are in the same set
    println!("0 and 1 are in the same set: {}", uf.find(0) == uf.find(1));
    println!("2 and 4 are in the same set: {}", uf.find(2) == uf.find(4));
}

