use std::collections::BTreeSet;

fn main() {
    // Create a new BTreeSet
    let mut ordered_set = BTreeSet::new();

    // Insert elements into the ordered set
    ordered_set.insert(3);
    ordered_set.insert(1);
    ordered_set.insert(4);
    ordered_set.insert(2);

    // Iterate over the ordered set
    for value in &ordered_set {
        println!("{}", value);
    }

    // Check if the ordered set contains a value
    if ordered_set.contains(&2) {
        println!("The ordered set contains the value 2");
    } else {
        println!("The ordered set does not contain the value 2");
    }

    // Remove a value from the ordered set
    ordered_set.remove(&1);

    // Check if the ordered set is empty
    if ordered_set.is_empty() {
        println!("The ordered set is empty");
    } else {
        println!("The ordered set is not empty");
    }
}
