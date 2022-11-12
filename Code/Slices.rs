fn main() {
  let mut s = String::from("hello");
  let word = first_word(&s); // 5
  
  s.clear(); // empties the "String", making it equal to "" 
  
  let mut my_word = String::from("Hello, World!");
  let another_word =  first_word(&my_word); // 6 
  
  
  // Slicing
  // A "string slice" allows us to access a portion of the "String" 
  // without having to have a full access to the whole "String" 
  let hello_part = &s[0..5];
  let world_part = &s[7..12];
}

fn first_word(word: &String) -> usize {
  let bytes = word.as_bytes(); // conversion of string to bytes to check for spaces 
  
  for (i, &item) in bytes.iter().enumerate() { // we create an iterator for our bytes using the "iter()" method and 
    // "enumerate()" wraps the result of "iter()" method and returns each element as part of a tuple instead
    // the first element is an index and the second is a reference to the element
    if item == b' ' { // we search for the space using the byte literal syntax
      return i; // return the current position when we find space 
    } 
  }
  s.len() //  if there are no spaces in the string, we return its length
}
