fn main() {
  let vector1: Vec<u32> = Vec::new(); 
  let vector2 = vec![1, 2, 3];
  
  let mut vector3 = Vec::new();
  
  vector3.push(5);
  vector3.push(23);
  vector3.push(1021);
  
  let vector4 = vec![1, 2, 3, 4, 5];
  let third: &i32 = &vector4[2];
  println!("The third element is {}", third);
  
  let third: Option<i32> = vector4.get(2);
  match third {
    Some(third) => println!("The third element is {}", third);
    None => println!("There is no third element");
  }
}
