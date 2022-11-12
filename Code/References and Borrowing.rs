fn main() {
  let s = String::from("hi"); 
  let len = calculate_length(&s); // "&s" creates a reference of the variable 's', i.e.
  // it refers to its value without taking ownership of it
  
  println!("The length of '{}' is {}.", s, len);
  
  
  let mut s = String::from("hi");
  //
  // You are not allowed to have multiple mutable references to the same variable 
  // let r1 = &mut s;
  // let r2 = &mut s;
  //
  // println!("{}, {}", r1, r2);
  change(&mut s); // we call the function by using a mutable reference 
  
  
  // To avoid the problem with multiple mutable references to 's'
  // we can use curly brackets (create a scope) for each one of the references to 's'
  // so when we are outside the current scope, it will be safe to make a new reference
  
  {
    let r1 = &mut s;
  } 
  
  let r2 = &mut s;
  
}

fn calculate_length(s: &String) -> usize { // 's' is a reference to a "String"
  s.len()
} // 's' goes out of scope and since it does not have ownership of the it refers to,
// it is not dropped

fn change(s: &mut String) { // the function mutates the value that borrows
  s.push_str(", world");
} 
