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
    
    // Validate the code
    if !is_valid_cpp(&code) {
        println!("Validation failed. Aborting analysis.");
        return;
    }

    // Regex patterns for identifying common constructs
    let for_loop_pattern = Regex::new(r#"for\s*\([^;]+;\s*[^;]+;\s*[^)]+\)"#).unwrap();
    let while_loop_pattern = Regex::new(r#"while\s*\([^)]+\)"#).unwrap();
    let if_statement_pattern = Regex::new(r#"if\s*\([^)]+\)"#).unwrap();
    let function_definition_pattern = Regex::new(r#"[\w<>]+\s+\w+\s*\([^)]*\)\s*{\s*"#).unwrap();

    // Regex patterns for identifying common constructs
    let for_loop_pattern = Regex::new(r#"for\s*\([^;]+;\s*[^;]+;\s*[^)]+\)"#).unwrap();
    let while_loop_pattern = Regex::new(r#"while\s*\([^)]+\)"#).unwrap();
    let if_statement_pattern = Regex::new(r#"if\s*\([^)]+\)"#).unwrap();
    let function_definition_pattern = Regex::new(r#"[\w<>]+\s+\w+\s*\([^)]*\)\s*{\s*"#).unwrap();

    // Display results
    println!("Occurrences of for loops: {}", for_loop_count);
    println!("Occurrences of while loops: {}", while_loop_count);
    println!("Occurrences of if statements: {}", if_statement_count);
    println!("Occurrences of function definitions: {}", function_definition_count);
}

// Function to perform additional validations on C++ code
fn is_valid_cpp(code: &str) -> bool {
    // Validation rules
    let no_tabs_or_spaces_at_beginning = code.lines().all(|line| line.trim_start().is_empty());
    let no_unmatched_braces = code.chars().filter(|&c| c == '{').count() == code.chars().filter(|&c| c == '}').count();
    let no_unmatched_parentheses = code.chars().filter(|&c| c == '(').count() == code.chars().filter(|&c| c == ')').count();
    let no_single_line_comments = !code.lines().any(|line| line.trim_start().starts_with("//"));
    let no_multiple_line_comments = !code.contains("/*") && !code.contains("*/");
    let no_empty_lines = code.lines().all(|line| !line.trim().is_empty());
    let no_goto_statements = !code.contains("goto");
    let no_using_namespace_directive = !code.contains("using namespace");
    let no_function_prototypes = code.lines().all(|line| !line.contains(';') || line.contains('(') || line.contains(')'));
    let proper_indentation = code.lines().all(|line| line.starts_with(' '));
    let no_static_globals = !code.contains("static");
    let proper_function_definitions = is_proper_function_definitions(code);
    let proper_variable_naming = is_proper_variable_naming(code);    

    if !no_tabs_or_spaces_at_beginning {
        eprintln!("Validation failed: Remove leading spaces or tabs from the beginning of lines.");
        return false;
    }

    if !no_unmatched_braces {
        eprintln!("Validation failed: Check for unmatched braces.");
        return false;
    }

    if !no_unmatched_parentheses {
        eprintln!("Validation failed: Check for unmatched parentheses.");
        return false;
    }
}
