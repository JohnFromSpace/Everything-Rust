mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist();
  }
}

use crate::front_of_house::hosting::add_to_waitlist();

pub fn eat_at_restaurant() {
  add_to_waitlist();  
}

///////////

use std::fmt;
use std::io;

fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}

///////////

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}

///////////



///////////

fn main() {}
