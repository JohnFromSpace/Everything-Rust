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
  
  // we can have more functions inside the current block
}

// we can have multiple "impl" blocks with that typename
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

// we can create a "square" from "Rectangle"
impl Rectangle {
  fn square(size: u32) -> Self { // an associated function
    // an associated function acts like a constructor 
    // that will create a new instance
    Self {
      width: size,
      height: size,
    }
  }
}


fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 20,
  };
  println!("The area of the rectangle is {}.", rect1.area()); 

}
