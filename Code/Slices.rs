// String literals are Slices.
// If we have a String, we can pass a slice of that String or a reference to the String.

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
  
  
  let a_word = String::from("word");
  let slice = &a_word[0..2]; // this is the same as
  let slice = &a_word[..2];
  
  let len_word = a_word.len(); 
  
  let slice = &a_word[2..len]; // this is the same as 
  let slice = &a_word[2..];
  
  let slice  = &a_word[0..len]; //this is the same as
  let slice = &a_word[..];
  
  let my_string = String::from("hello world");
  
  // `first_word` works on slices of `String`s, whether partial or whole
  let word = first_word(&my_string[0..6]);
  let word = first_word(&my_string[..]);
  
  // `first_word` also works on references to `String`s, which are equivalent
  // to whole slices of `String`s
  let word = first_word(&my_string);
  
  let my_string_literal = "hello world";
  
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

fn another_first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return s[0..i]; // current position of "space"
    } 
  }
  &s[..] // length of 's'
}
