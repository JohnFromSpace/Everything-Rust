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
        let parameters = parameters
            .into_iter()
            .map(|(n, t)| (n.to_string(), t.to_string()))
            .collect(); 
        RustFunction {
            name: name.to_string(),
            parameters,
            return_type: return_type.to_string(),
            body: body.to_string(),
        }
    }
}

trait LanguageConverter {
    
}
