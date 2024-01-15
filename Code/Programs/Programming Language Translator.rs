use std::fmt::Write;

#[derive(Debug)]
struct RustFunction {
    name: String,
    parameters: Vec<(String, String)>,
    return_type: String,
    body: String,
}

impl RustFunction {
    fn new(name: &str, parameters: Vec<(&str, &str)>, return_type: &str, body: &str) -> Self {
        
    }
}
