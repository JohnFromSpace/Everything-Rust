use std::collections::HashMap;

fn main() {
  let mut hm = HashMap::new(); 
  hm.insert(String::from("A"), 10); // "A" is associated/paired with 10
  hm.insert(String::from("B"), 2); // "B" is paired with 2
  
  // access a HashMap's value it stores 
  let name = String::from("B");
  let hash_map = hm.get(&name).copy().unwrap_or(0); // 2
  
  
}
