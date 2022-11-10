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
  
  
  
    
}
