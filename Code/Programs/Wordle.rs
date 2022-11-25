use colored::*;
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;

const ALL_WORDS: &str = include_str!("words.txt");
const WORD_LENGTH: usize = 5;
const MAX_TRIES: usize = 6;

fn fix_word(word: &str) -> String {
  word.trim().to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect
}

fn list() -> Vec<String> {
  ALL_WORDS.split('\n').skip(2).map(fix_word).filter(|line| line.len() == WORD_LENGTH).collect()
}

struct Game {
  dictionary: Vec<String>,
  word: String,
  guessed_word_letters: HashSet<char>,
  guesses: Vec<String>,
}

impl Game {
  fn new() -> Self {
    let mut rng = RandomNumberGenerator::new();
    let dictionary = list();
    let word = rng.random_slice_entry(&dictionary).unwrap().clone();
    Self {
      dictionary, 
      word,
      guessed_word_letters: HashSet::new(),
      guesses: Vec::new(),
    }
  }
  
  
  
}

fn main() {}
