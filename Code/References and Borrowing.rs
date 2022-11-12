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
  
  
  // There is no problem in using multiple immutable references at the same time 
  let iR1 = &s;
  let iR2 = &s;
  
  println!("{} and {}", iR1, iR2); // the variables "iR1" and "iR2" will not be used anymore
  // This allows us to create a mutable reference to 's'
  let mutable_reference = &mut s;
  println!("{}", mutable_reference);
  
}

fn calculate_length(s: &String) -> usize { // 's' is a reference to a "String"
  s.len()
} // 's' goes out of scope and since it does not have ownership of the it refers to,
// it is not dropped

fn change(s: &mut String) { // the function mutates the value that borrows
  s.push_str(", world");
} 

fn no_dangle() -> String {
  let s = String::from("hello");
  
  s
} // this function creates a variable 's' and returns the "String" directly without having to 
// deallocate anything as it would happen if we called the function by reference
// since such actions would cause the variable 's' to be created and existed only inside the scope of the function
// once the function would end
// 's' would be deallocated and nothing would be returned
