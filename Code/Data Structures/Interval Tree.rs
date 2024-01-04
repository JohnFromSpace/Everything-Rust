use std::cmp::Ordering;

#[derive(Debug)]
struct Interval {
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct IntervalTreeNode {
    interval: Interval,
    max_end: i32,
    left: Option<Box<IntervalTreeNode>>,
    right: Option<Box<IntervalTreeNode>>,
}

impl IntervalTreeNode {
    fn new(interval: Interval) -> Self {
        let max_end = interval.end;
        IntervalTreeNode {
            interval,
            max_end,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct IntervalTree {
    root: Option<Box<IntervalTreeNode>>,
}

impl IntervalTree {
    pub fn new() -> Self {
        IntervalTree { root: None }
    }

    pub fn insert(&mut self, interval: Interval) {
        self.root = self.insert_recursive(self.root.take(), interval);
    }

    fn insert_recursive(&mut self, node: Option<Box<IntervalTreeNode>>, interval: Interval) -> Option<Box<IntervalTreeNode>> {
        match node {
            None => Some(Box::new(IntervalTreeNode::new(interval))),
            Some(mut current) => {
                let comparison = compare_intervals(&interval, &current.interval);

                match comparison {
                    Ordering::Less => {
                        current.left = self.insert_recursive(current.left.take(), interval);
                    }
                    Ordering::Greater | Ordering::Equal => {
                        current.right = self.insert_recursive(current.right.take(), interval);
                    }
                }

                current.max_end = current.max_end.max(interval.end);

                Some(current)
            }
        }
    }

    pub fn search_overlap(&self, query: Interval) -> Vec<&Interval> {
        let mut result = Vec::new();
        self.search_overlap_recursive(&self.root, query, &mut result);
        result
    }

    fn search_overlap_recursive(&self, node: &Option<Box<IntervalTreeNode>>, query: Interval, result: &mut Vec<&Interval>) {
        if let Some(current) = node {
            if intervals_overlap(&current.interval, &query) {
                result.push(&current.interval);
            }

            if let Some(ref left_child) = current.left {
                if left_child.max_end >= query.start {
                    self.search_overlap_recursive(&current.left, query, result);
                }
            }

            if query.start <= current.interval.start && query.end >= current.interval.start {
                self.search_overlap_recursive(&current.right, query, result);
            }
        }
    }
}

fn compare_intervals(interval1: &Interval, interval2: &Interval) -> Ordering {
    if interval1.start < interval2.start {
        Ordering::Less
    } else if interval1.start > interval2.start {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn intervals_overlap(interval1: &Interval, interval2: &Interval) -> bool {
    interval1.start <= interval2.end && interval2.start <= interval1.end
}

fn main() {
    let mut interval_tree = IntervalTree::new();

    interval_tree.insert(Interval { start: 15, end: 20 });
    interval_tree.insert(Interval { start: 10, end: 30 });
    interval_tree.insert(Interval { start: 5, end: 12 });
    interval_tree.insert(Interval { start: 25, end: 30 });

    let query_interval = Interval { start: 17, end: 28 };
    let overlapping_intervals = interval_tree.search_overlap(query_interval);

    println!("Overlapping Intervals: {:?}", overlapping_intervals);
}
