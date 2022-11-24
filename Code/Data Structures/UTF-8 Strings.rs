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
  let ss1 = String::from("tic");
  let ss2 = String::from("tac");
  let ss3 = String::from("toe");
  let sss = ss1 + "-" + &ss2 + "-" + &ss3; // "tic-tac-toe"
  
  let sss1 = String::from("tic");
  let ssss = format!("{}-{}-{}", sss1, ss2, ss3); // "tic-tac-toe"
  
  // Access "String" element
  let hello = "hello";
  let answer = &hello[0]; // 'h'
  
  // Slicing Strings
  let hellos = "hellos";
  let shellos = &hellos[0..2];
  
  // Iteration
  for c in "hello".chars() {
    println!("{}", c);
    // h
    // e
    // l
    // l
    // o
  }
  
  for b in "hello".bytes() {
    println!("{}", b);
    // 104
    // 101
    // 108
    // 108 
    // 111
  }
  
}
