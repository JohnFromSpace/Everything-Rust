use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is {secret_number}.");
    
    println!("Please, input your guess.");
    
    let mut guess_number = String::new();
    
    io::stdin().read_line(&mut guess_number).expect("Failed to read line!");
}
