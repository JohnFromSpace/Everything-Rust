use std::collections::BTreeMap;

fn main() {
    // Create a new BTreeMap
    let mut ordered_map = BTreeMap::new();

    // Insert key-value pairs
    ordered_map.insert("three", 3);
    ordered_map.insert("one", 1);
    ordered_map.insert("four", 4);
    ordered_map.insert("two", 2);

    // Iterate over the ordered map
    for (key, value) in &ordered_map {
        println!("{}: {}", key, value);
    }

    // Access elements by key
    if let Some(value) = ordered_map.get("two") {
        println!("The value associated with 'two' is: {}", value);
    }

    // Remove an element by key
    ordered_map.remove("one");

    // Check if the ordered map contains a key
    if ordered_map.contains_key("one") {
        println!("The ordered map contains the key 'one'");
    } else {
        println!("The ordered map does not contain the key 'one'");
    }
}
