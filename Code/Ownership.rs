fn main() {
  let s = "hello"; // valid until the end of the scope of the main function
  
  {
     let s = "hello"; // valid for now
  } // outside the scope 's' is no longer valid

}
