extern crate racer;
extern crate cppcomplete;
extern crate syntect;
extern crate rustfmt;

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

struct CodeSnippet {
    language: Language,
    content: String,
}

enum Language {
    Rust,
    Cpp,
}

struct UserPreferences {
    perform_analysis: bool,
    syntax_highlighting: bool,
    perform_formatting: bool,
}

fn write_code_to_file(language: Language, code: &str, file_path: &str) {
    fs::write(file_path, code).expect("Failed to write to the file");    
}

fn get_rust_completions(file_path: &str, line: usize, column: usize) -> racer::Result<Vec<racer::Match>> {
    racer::complete_from_file(file_path, line, column)    
}

fn get_cpp_completions(file_path: &str, line: usize, column: usize) -> cppcomplete::Result<Vec<cppcomplete::Completion>> {
    cppcomplete::get_completions(file_path, line, column)    
}

fn analyze_code(language: Language, code: &str) {
    match language {
        Language::Rust => {
            // Perform Rust-specific analysis if needed
            // For example, extract variable types, function signatures, etc.
            println!("Performing Rust analysis:");
            // TODO: Add Rust analysis logic
        }
        Language::Cpp => {
            // Perform C++-specific analysis if needed
            // For example, extract variable types, function signatures, etc.
            println!("Performing C++ analysis:");
            // TODO: Add C++ analysis logic
        }
    }
}

fn display_completions(language: Language, completions: Vec<String>) {
    match language {
        Language::Rust => {
            println!("Rust Completions:");
        }
        Language::Cpp => {
            println!("C++ Completions:");
        }
    }

    for (index, completion) in completions.iter().enumerate() {
        println!("{}. {}", index + 1, completion);
    }
}

fn show_additional_info(language: Language, index: usize, completions: Vec<String>) {
    match language {
        Language::Rust => {
            // Display additional information for Rust completions if available
            if let Ok(match_) = get_rust_completions("temp_file", 5, 17) {
                if index < match_.len() {
                    println!("Additional Info for Rust Completion {}: {:?}", index + 1, match_[index]);
                }
            }
        }
        Language::Cpp => {
            // Display additional information for C++ completions if available
            if let Ok(completion) = get_cpp_completions("temp_file", 5, 17) {
                if index < completion.len() {
                    println!("Additional Info for C++ Completion {}: {:?}", index + 1, completion[index]);
                }
            }
        }
    }
}

fn perform_syntax_highlighting(code: &str) {
    let theme = ThemeSet::load_defaults().themes["base16-ocean.dark"];
    let syntax_set = SyntaxSet::load_defaults_nonewlines();    

    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap(); // Assume Rust syntax for now
    let mut highlighter = HighlightLines::new(syntax, &theme);

    for line in code.lines() {
        let ranges = highlighter.highlight(line);
        let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);
        println!("{}", escaped);
    }
}

fn format_code(code: &str) -> String {
    
}
