use std::collections::HashSet;

#[derive(Debug)]
struct CustomSet<T> {
    inner: HashSet<T>,
}

impl<T> CustomSet<T>
where
    T: Eq + Hash,
{
    fn new() -> Self {
        CustomSet {
            inner: HashSet::new(),
        }
    }

    fn insert(&mut self, value: T) {
        self.inner.insert(value);
    }

    fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    fn remove(&mut self, value: &T) {
        self.inner.remove(value);
    }

    fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut result = self.clone();
        result.inner.extend(&other.inner);
        result
    }

    fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut result = CustomSet::new();
        for value in &self.inner {
            if other.contains(value) {
                result.insert(value.clone());
            }
        }
        result
    }

    fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut result = CustomSet::new();
        for value in &self.inner {
            if !other.contains(value) {
                result.insert(value.clone());
            }
        }
        result
    }
}

fn main() {
    let mut set1 = CustomSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    let mut set2 = CustomSet::new();
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);

    let union_result = set1.union(&set2);
    println!("Union: {:?}", union_result);

    let intersection_result = set1.intersection(&set2);
    println!("Intersection: {:?}", intersection_result);

    let difference_result = set1.difference(&set2);
    println!("Difference: {:?}", difference_result);
}
