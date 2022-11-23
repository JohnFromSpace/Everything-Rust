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

mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant_again() {
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

fn main() {}
