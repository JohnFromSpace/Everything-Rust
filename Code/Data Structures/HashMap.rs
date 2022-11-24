use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new(); 
    hm.insert(String::from("A"), 10); // "A" is associated/paired with 10
    hm.insert(String::from("B"), 2); // "B" is paired with 2

    // access a HashMap's value it stores 
    let name = String::from("B");
    let hash_map = hm.get(&name).copy().unwrap_or(0); // 2

    for(key, value) in &hm {
      println!("{}: {}", key, value); 
      // "A: 10"
      // "B: 2"
    }

    // Ownership
    let field_name = String::from("Favourite");
    let filed_value = String::from("Banana");

    let mut map = HashMap::new(0;
    map.insert(field_name, field_value);  

    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores); // "Blue: 20"
           
    
}
