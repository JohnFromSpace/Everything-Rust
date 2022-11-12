fn main() {

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
