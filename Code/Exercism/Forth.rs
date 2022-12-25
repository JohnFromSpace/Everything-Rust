pub type Value = i32;
type Result<T> = std::result::Result<T, Error>;

use std::{convert::TryInto, str::FromStr};
pub type ForthResult = Result<()>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

struct Definition {
    name: String,
    body: Vec<Instruction>,
}

#[derive(Default)]
pub struct Forth {
    dict: Vec<Definition>,
    stack: Vec<Value>,
}

fn parse_builtin(word: &str) -> Result<Instruction> {
    match word {
        "+" => Ok(Instruction::Add),
        "-" => Ok(Instruction::Sub),
        "*" => Ok(Instruction::Mul),
        "/" => Ok(Instruction::Div),
        "DUP" => Ok(Instruction::Dup),
        "SWAP" => Ok(Instruction::Swap),
        "DROP" => Ok(Instruction::Drop),
        "OVER" => Ok(Instruction::Over),
        _ => if let Ok(num) = Value::from_str(word) {
            Ok(Instruction::Number(num))
        } 
        
        else {
            eprintln!("parse_builtin failed");
            Err(Error::UnknownWord)
        }
    }
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
