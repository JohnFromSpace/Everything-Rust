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
  
  fn display_guesses(&mut self) {
    self.guesses.iter().enumerate().for_each(|guess_number, guess|) {
            print!("{}: ", guess_number+1);
            guess.chars().enumerate().for_each(|(pos, c)| {
                let display = if self.word.chars().nth(pos).unwrap() == c {
                    format!("{c}").bright_green()
                } 
              
                else if self.word.chars().any(|wc| wc == c) {
                    format!("{c}").bright_yellow()
                } 
                
                else {
                    self.guessed_word_letters.insert(c);
                    format!("{c}").red()
                };
                print!("{display}");
            });
            println!();
        })
 
  fn display_invalid_letters(&self) {
    if !self.guessed_word_letters.is_empty() {
      print!("Letters not in word: ");
      self.guessed_word_letters.iter().for_each(|letter| print!("{letter} "));
      println!();
    }
  }
  
  fn is_game_over(&self, guess: &str) -> bool {
    let n_tries = self.guesses.len();
    if guess == self.word {
      println!("{}", "Correct! You guessed the word in {n_tries} tries.", bright_green());
      true
    }
    else if n_tries >= MAX_TRIES {
      println!("{}", format!("You ran out of tries! The word was {}.", self.word, bright_red()));
      true
    }
    else {
      false
    }
  }
  
  
  
  
}

fn main() {}
