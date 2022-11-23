mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // Absolute path
  // "crate" is the root crate of the mod tree 
  crate::front_of_house::hosting::add_to_waitlist();
  
  // Relative path
  // we can refer to "front_of_house" from "eat_at_restaurant"
  front_of_house::hosting::add_to_waitlist();
}

fn main() {}
