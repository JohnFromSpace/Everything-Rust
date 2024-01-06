use std::collections::HashMap;

fn main() {
    // Create a new HashMap with keys and values of type String
    let mut unordered_map: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs into the HashMap
    unordered_map.insert("one".to_string(), 1);
    unordered_map.insert("two".to_string(), 2);
    unordered_map.insert("three".to_string(), 3);

    // Access values using keys
    if let Some(value) = unordered_map.get("two") {
        println!("The value for key 'two' is: {}", value);
    } else {
        println!("Key 'two' not found.");
    }

    // Iterate over key-value pairs
    for (key, value) in &unordered_map {
        println!("Key: {}, Value: {}", key, value);
    }

    // Remove a key-value pair
    unordered_map.remove("two");

    // Check if a key is present
    if unordered_map.contains_key("two") {
        println!("Key 'two' is present.");
    } else {
        println!("Key 'two' is not present.");
    }
}

