use std::fs;
use regex::Regex;

fn main() {
    // Read the C++ code from a file
    let code = match fs::read_to_string("your_file.cpp") { 
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    
}
