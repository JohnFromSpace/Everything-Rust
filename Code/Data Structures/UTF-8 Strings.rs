fn main() {
  let mut s = String::new();
  
  let data = "initial data";
  let s = data.to_string();
   
  let s = "intial contents".to_string();
  
  let s = String::from("intial contents");
  
  // Appending a String to String
  let s1 = String::from("Hello, ");
  let s2 = String::from("World!");
  s1.push_str(s2);
  println!("{}", s1); // "Hello, World!"
  
  let s3 = String::from("Foo");
  let s4 = "Bar";
  s3.push_str(s4);
  println!("{}", s3); // "FooBar"
  
  let s = String::from("My name is ");
  s.push('L');
  println!("{}", s);
  
  let more_strings1 = String::from("Hello, ");
  let more_strings2 = Strinng::from("World!");
  let more_strings3 = more_strings1 + &more_strings2; // "more_strings1" has been moved here 
  // and can no longer be used 
  
  // Concatenation with '+' operator
  
}
