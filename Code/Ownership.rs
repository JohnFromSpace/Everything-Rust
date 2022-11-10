fn main() {
  let s = "hello"; // valid until the end of the scope of the main function
  
  {
     let s = "hello"; // valid for now
  } // outside the scope 's' is no longer valid
  
  let s = String::from("hello"); // this type of "String" Rust has, 
  // allows us to allocate memory on heap which stores an amount of text 
  // that is unknown to us at compile time
  // A string can be created from a string literal by using the function "from"
  // The double colon "::" allows us to namespace this particular "from" function
  // under the "String" type rather than using a different name
  
  let mut s = String::from("hello");
  s.push_str(", world"); // push_str() appends a literal to a string 
  println!("{}", s); // "hello, world"
  
  {
    let s = String::from("hello");
  } // once the program leaves this scope, 's' is no longer valid
  
  let x = 5; 
  let y = x; // makes a copy of the value in 'x' and assigns it 
  // it means that there are two variables with integer values of 5
  // and those values are pushed on the stack  
  
  // A string is made of 3 parts:
  // a pointer to the memory that holds all the contents of the string;
  // a length;
  // a capacity.
  // 
  // All of these are stored on the stack
  // 
  // Length is the current amount of memory, in bytes, that the "String" is using
  // 
  // Capacity is the total amount of memory, in bytes, that the "String" has received from the allocator
  let s1 = String::from("hello");
  let s2 = s1; // the pointer, the length and the capacity of 's1' on the stack are ONLY copied
  // none of the data on the heap that the pointer refers to is copied
  // Such process of assigning 's1' value to 's2' can be considered a "shallow copy"
  // but since Rust invalidates the first variable that makes the "shallow copy" more like "move"
  // 
  // To implement a "deep copy", i.e. to copy the data of 's1' on the heap to 's2' 
  // we can use the method "clone", instead
  
  let s1 = String::from("hello"); // data is allocated on heap
  let s2 = s1.copy(); // heap data of 's1' is copied to 's2'
  println!("s1 = {}, s2 = {}", s1, s2);
  
  // Making copies of integer-type values is pointless 
  // since they already have a definitive size at compile time
  // and are entirely stored on the stack, so that makes the copy fast
  // it means that there is no need to use method "clone" on 's1' because
  // we will achieve nothing different in terms of efficiency
  
  // Multiple values can be returned by a function by using a tuple
  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  
  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}
