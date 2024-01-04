use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct DisjointSetElement<T> {
    value: T,
    rank: usize,
    parent: T,
}

#[derive(Debug)]
pub struct DisjointSet<T: PartialEq + Eq + Hash + Clone> {
    elements: HashMap<T, DisjointSetElement<T>>,
}

impl<T: PartialEq + Eq + Hash + Clone> DisjointSet<T> {
    pub fn new() -> Self {
        DisjointSet {
            elements: HashMap::new(),
        }
    }

    pub fn make_set(&mut self, value: T) {
        if !self.elements.contains_key(&value) {
            let element = DisjointSetElement {
                value: value.clone(),
                rank: 0,
                parent: value.clone(),
            };
            self.elements.insert(value, element);
        }
    }

    pub fn find_set(&mut self, value: T) -> Option<T> {
        if let Some(element) = self.elements.get(&value).cloned() {
            if element.value != element.parent {
                let root = self.find_set(element.parent.clone()).unwrap();
                self.elements.get_mut(&value).unwrap().parent = root.clone();
                Some(root)
            } else {
                Some(value)
            }
        } else {
            None
        }
    }

    pub fn union_sets(&mut self, value1: T, value2: T) {
        let root1 = self.find_set(value1.clone());
        let root2 = self.find_set(value2.clone());

        if let (Some(root1), Some(root2)) = (root1, root2) {
            if root1 != root2 {
                let rank1 = self.elements[&root1].rank;
                let rank2 = self.elements[&root2].rank;

                if rank1 < rank2 {
                    self.elements.get_mut(&root1).unwrap().parent = root2.clone();
                } else if rank1 > rank2 {
                    self.elements.get_mut(&root2).unwrap().parent = root1.clone();
                } else {
                    self.elements.get_mut(&root1).unwrap().parent = root2.clone();
                    self.elements.get_mut(&root2).unwrap().rank += 1;
                }
            }
        }
    }
}

fn main() {
    let mut disjoint_set = DisjointSet::new();

    disjoint_set.make_set("A");
    disjoint_set.make_set("B");
    disjoint_set.make_set("C");

    println!("Initial sets: {:?}", disjoint_set);

    disjoint_set.union_sets("A", "B");
    disjoint_set.union_sets("B", "C");

    println!("Sets after union: {:?}", disjoint_set);

    println!("Find set for 'A': {:?}", disjoint_set.find_set("A")); // Output: Some("C")
    println!("Find set for 'B': {:?}", disjoint_set.find_set("B")); // Output: Some("C")
    println!("Find set for 'C': {:?}", disjoint_set.find_set("C")); // Output: Some("C")
}
