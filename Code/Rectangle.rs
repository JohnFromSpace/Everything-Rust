struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let width = 30;
  let height = 20;
  println!("The area of the rectangle is {}.", area(width, height)); // 600
  
  let rect = (30, 40);
  println!("The area of the rectangle is {}.", area_tuples(rect)); // 120
  
  let rectangle = Rectangle {
    width: 30, 
    height: 30, 
  }
  
  println!("The area is {}.", area_struct(&rectangle)); // 900
  
}

fn area(w: u32, h: u32) -> u32 {
  w * h
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
