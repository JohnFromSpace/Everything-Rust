pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
  
pub struct Forth;
use std::{convert::TryInto, str::FromStr};
pub type ForthResult = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        std::default::Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut iter = input.split_ascii_whitespace();
        while let Some(word) = iter.next() {
            self.parse_word(word, &mut iter);
        }
        Ok(())
    }
}
