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

use syn::{Item, parse_quote};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use regex::Regex;

// Add the following dependencies to your Cargo.toml:
// syn = "1.0"
// quote = "1.0"
// proc-macro2 = "1.0"

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
            analyze_rust_code(code);
        }
        Language::Cpp => {
            // Perform C++-specific analysis if needed
            // For example, extract variable types, function signatures, etc.
            println!("Performing C++ analysis:");
            analyze_cpp_code(code);
        }
    }
}

fn analyze_rust_code(code: &str) {
    if let Ok(syntax_tree) = syn::parse_file(code) {
        for item in syntax_tree.items {
            match item {
                Item::Fn(ref func) => {
                    println!("Found Rust function: {}", func.sig.ident);
                    analyze_rust_function(func);    
                }
                Item::Struct(ref st) => {
                    println!("Found Rust struct: {}", st.ident);
                    analyze_rust_struct(st);    
                }
                _ => {}
            }   
        }
    } else {
        eprintln!("Failed to parse Rust code");
    }   
}

fn analyze_rust_function(func: &ItemFn) {
    println!("Found Rust function: {}", func.sig.ident);

    // Display function visibility
    if let Some(visibility) = &func.vis {
        match visibility {
            Visibility::Public(_) => println!("Function is public."),
            Visibility::Crate(_) => println!("Function has crate visibility."),
            Visibility::Restricted(_) => println!("Function has restricted visibility."),
            Visibility::Inherited => {} // Do nothing for inherited visibility    
        }   
    }

    // Display function parameters
    for input in &func.sig.inputs {
        if let Type::Path(TypePath { path, .. }) = &input.ty {
            let param_name = &input.pat;
            let param_type = &path.segments.last().unwrap().ident;
            println!("Parameter {}: {:?}", quote! {#param_name}, param_type);    
        }
    }

    // Display return type
    if let Some(return_type) = &func.sig.output {
        match return_type {
            syn::ReturnType::Default => println!("Function has no return type."),
            syn::ReturnType::Type(_, ty) => {
                if let Type::Path(TypePath { path, .. }) = &**ty {
                    let return_type = &path.segments.last().unwrap().ident;
                    println!("Return Type: {:?}", return_type);
                }
            }    
        }
    }
}

fn analyze_rust_struct(struct_item: &ItemStruct) {
    println!("Found Rust struct: {}", struct_item.ident);

    // Display struct visibility
    if let Some(visibility) = &struct_item.vis {
        match visibility {
            Visibility::Public(_) => println!("Struct is public."),
            Visibility::Crate(_) => println!("Struct has crate visibility."),
            Visibility::Restricted(_) => println!("Struct has restricted visibility."),
            Visibility::Inherited => {} // Do nothing for inherited visibility    
        }   
    }

    // Display struct fields and their types
    for field in &struct_item.fields {
        if let Some(ident) = &field.ident {
            let field_type = &field.ty;
            println!("Struct field {}: {:?}", ident, quote! {#field_type});
        }
    }

    // Generate code snippet for struct initialization
    let struct_name = &struct_item.ident;
    let struct_fields: Vec<Ident> = struct_item.fields.iter().filter_map(|f| f.ident.clone()).collect();
    let init_code = quote! {
        let instance = #struct_name {
            #(#struct_fields: Default::default()),*
        };
    };

    println!("Code snippet for struct initialization:\n{}", init_code);
}

fn analyze_cpp_code(code: &str) {
    let function_declaration_regex = regex::Regex::new(r"\b(?:\w+\s+){0,2}(\w+)\s*\([^)]*\)\s*{").unwrap();  

    for capture in function_declaration_regex.captures_iter(code) {
        if let (Some(func_name), Some(template_params), Some(params_str), const_modifier) =
            (capture.get(1), capture.get(2), capture.get(3), capture.get(4)) {
            let template_params = template_params.as_str();
            let params: Vec<&str> = params_str.as_str().split(',').map(|p| p.trim()).collect();

            println!("Found C++ function: {}", func_name.as_str());

            // Display template parameters
            if !template_params.is_empty() {
                println!("Template Parameters: {}", template_params);
            }

            // Display function parameters
            for param in &params {
                let param_parts: Vec<&str> = param.split_whitespace().collect();
                if param_parts.len() == 2 {
                    let param_type = param_parts[0];
                    let param_name = param_parts[1];
                    println!("Parameter {}: {:?}", param_name, param_type);
                }
            }

            // Display const modifier
            if const_modifier.is_some() {
                println!("Const Modifier: const");
            }

            // Extract and display return type
            let return_type = extract_return_type(code, capture.start());
            println!("Return Type: {:?}", return_type);

            // Generate C++ function signature code snippet
            let func_signature = quote! {
                #func_name#template_params(#(#parse_quote! { #params }),*) #const_modifier
            };

            println!("C++ Function Signature Code Snippet:\n{}", func_signature);
        }    
    }
}

fn extract_return_type(code: &str, start: usize) -> Option<String> {
    let mut depth = 0;
    let mut found_colon = false;
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
    match rustfmt::run_rustfmt(rustfmt::Config::default(), Some("stdin".to_owned()), code.into()) {
        Ok(result) => result.1,
        Err(err) => {
            eprintln!("Error formatting code: {}", err);
            code.to_owned()
        }
    }    
}

fn get_user_preferences() -> UserPreferences {
    // Allow the user to interactively choose preferences
    println!("Do you want to perform code analysis? (y/n):");
    let perform_analysis = read_yes_no_input();  

    println!("Do you want to enable syntax highlighting? (y/n):");
    let syntax_highlighting = read_yes_no_input();

    println!("Do you want to perform code formatting? (y/n):");
    let perform_formatting = read_yes_no_input();

    UserPreferences {
        perform_analysis,
        syntax_highlighting,
        perform_formatting,
    }
}

fn read_yes_no_input() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_lowercase() == "y"        
}

#[derive(Debug)]
enum SyntaxTheme {
    Default,
    Ocean,
    Custom(String),
}

fn choose_syntax_theme() -> SyntaxTheme {
    println!("Choose a syntax highlighting theme:");
    println!("1. Default");
    println!("2. Ocean");
    println!("3. Custom");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<u32>() {
        Ok(choice) => match choice {
            1 => SyntaxTheme::Default,
            2 => SyntaxTheme::Ocean,
            3 => {
                println!("Enter the custom theme name:");
                let mut theme_name = String::new();
                io::stdin().read_line(&mut theme_name).expect("Failed to read line");
                SyntaxTheme::Custom(theme_name.trim().to_owned())
            }
            _ => SyntaxTheme::Default, // Default theme if an invalid choice is made
        },
        Err(_) => SyntaxTheme::Default, // Default theme if parsing fails
    }
}

fn get_syntax_theme(theme_name: &str) -> Option<Style> {
    let theme_set = ThemeSet::load_defaults();
    theme_set.themes.get(theme_name).map(|theme| theme.settings.selection)    
}

fn perform_syntax_highlighting(code: &str, theme: SyntaxTheme) {
    match theme {
        SyntaxTheme::Default => perform_default_syntax_highlighting(code),
        SyntaxTheme::Ocean => perform_ocean_syntax_highlighting(code),
        SyntaxTheme::Custom(theme_name) => {
            if let Some(style) = get_syntax_theme(&theme_name) {
                perform_custom_syntax_highlighting(code, style);
            } else {
                eprintln!("Custom theme not found. Performing default syntax highlighting.");
                perform_default_syntax_highlighting(code);
            }
        }    
    }
}

fn perform_default_syntax_highlighting(code: &str) {
    // Default syntax highlighting logic
    println!("Performing Default Syntax Highlighting:");
    // TODO: Implement default syntax highlighting
}

fn perform_ocean_syntax_highlighting(code: &str) {
    // Ocean theme syntax highlighting logic
    println!("Performing Ocean Theme Syntax Highlighting:");
    // TODO: Implement Ocean theme syntax highlighting
}

fn perform_custom_syntax_highlighting(code: &str, style: Style) {
    // Custom theme syntax highlighting logic
    println!("Performing Custom Theme Syntax Highlighting:");
    // TODO: Implement custom theme syntax highlighting
}

fn main() {
    let mut code_snippet = String::new();

    // Allow the user to input code interactively
    println!("Enter your code (type 'done' on a new line to finish input):");  

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        if line.trim() == "done" {
            break;
        }

        code_snippet.push_str(&line);
    }

    // Determine the language based on the code
    let language = if code_snippet.contains("#include") {
        Language::Cpp
    } else {
        Language::Rust
    };

    // Temporary file path to store the code snippet
    let file_path = "temp_file";

    // Write the code snippet to a temporary file
    write_code_to_file(language.clone(), &code_snippet, file_path);

    // Get user preferences
    let preferences = get_user_preferences();

    // Perform analysis based on user preferences
    if preferences.perform_analysis {
        analyze_code(language.clone(), &code_snippet);
    }

    // Get completions based on the language
    let completions = match language {
        Language::Rust => get_rust_completions(file_path, 5, 17), // Adjust line and column based on your cursor position
        Language::Cpp => get_cpp_completions(file_path, 5, 17),   // Adjust line and column based on your cursor position
    };

    // Display completions
    match completions {
        Ok(completions) => {
            let completion_strings: Vec<String> = completions.iter().map(|c| c.name.clone()).collect();
            display_completions(language.clone(), completion_strings);

            // Allow the user to choose a completion
            println!("Enter the number of the completion you want to select:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if let Ok(index) = input.trim().parse::<usize>() {
                if index > 0 && index <= completions.len() {
                    show_additional_info(language, index - 1, completion_strings);
                } else {
                    eprintln!("Invalid selection index.");
                }
            } else {
                eprintln!("Invalid input. Please enter a number.");
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    // Get user preferences
    let preferences = get_user_preferences();

    // Choose syntax highlighting theme
    let syntax_theme = choose_syntax_theme();

    // Perform syntax highlighting based on user preferences
    if preferences.syntax_highlighting {
        perform_syntax_highlighting(&code_snippet, syntax_theme);
    }
    
    // Perform code formatting based on user preferences
    if preferences.perform_formatting {
        let formatted_code = format_code(&code_snippet);
        println!("Formatted Code:\n{}", formatted_code);
    }

    // Clean up the temporary file
    fs::remove_file(file_path).expect("Failed to remove the temporary file");
}
