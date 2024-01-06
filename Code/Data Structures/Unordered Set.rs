use std::collections::HashSet;

fn main() {
    // Create a new HashSet with elements of type i32
    let mut unordered_set: HashSet<i32> = HashSet::new();

    // Insert elements into the HashSet
    unordered_set.insert(1);
    unordered_set.insert(2);
    unordered_set.insert(3);

    // Check if an element is present
    if unordered_set.contains(&2) {
        println!("Element 2 is present in the set.");
    } else {
        println!("Element 2 is not present in the set.");
    }

    // Iterate over elements
    for &element in &unordered_set {
        println!("Element: {}", element);
    }

    // Remove an element
    unordered_set.remove(&2);

    // Check if an element is present after removal
    if unordered_set.contains(&2) {
        println!("Element 2 is present in the set.");
    } else {
        println!("Element 2 is not present in the set.");
    }
}

