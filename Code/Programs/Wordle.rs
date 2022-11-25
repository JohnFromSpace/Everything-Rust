use colored::*;
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;

const ALL_WORDS: &str = include_str!("words.txt");
const WORD_LENGTH: usize = 5;
const MAX_TRIES: usize = 6;

fn fix_word(word: &str) -> String {
  word.trim().to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect
}

fn words_list() -> Vec<String> {
  ALL_WORDS.split('\n').skip(2).map(fix_word).filter(|line| line.len() == WORD_LENGTH).collect()
}

struct Game {
  dictionary: Vec<String>,
  word: String,
  guessed_word_letters: HashSet<char>,
  guesses: Vec<String>,
}



fn main() {}
