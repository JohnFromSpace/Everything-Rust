#[derive(Debug)]

struct Rectangle {
  width: u32,
  height: u32,
}

// function "area" is defined within the context of "Rectangle",
// thus it will be associated with that type
// we use the keyword "impl" and after it we add the typename
impl Rectangle {
  fn area(&self) -> u32 { 
    self.width * self.height
  }  
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 20,
  };
  println!("The area of the rectangle is {}.", rect1.area()); 

}
