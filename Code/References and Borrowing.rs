fn main() {
  let s = String::from("hi"); 
  let len = calculate_length(&s); // "&s" creates a reference of the variable 's', i.e.
  // it refers to its value without taking ownership of it
  
  println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize { // 's' is a reference to a "String"
  s.len()
} // 's' goes out of scope and since it does not have ownership of the it refers to,
// it is not dropped
