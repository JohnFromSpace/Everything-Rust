fn main() {
  let width = 30;
  let height = 20;
  println!("The area of the rectangle is {}.", area(width, height)); // 600
  
  let rect = (30, 40);
  println!("The area of the rectangle is {}.", area_tuples(rect));
  
  
  
}

fn area(w: u32, h: u32) -> u32 {
  w * h
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}
