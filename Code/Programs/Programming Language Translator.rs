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
    fn convert(&self, rust_function: &RustFunction) -> Result<String, String>;
    fn details(&self) -> &'static LanguageDetails;    
}

trait LanguageDetails {
    fn name(&self) -> &'static str;
    fn file_extension(&self) -> &'static str;    
}

struct CodeConverter<'a> {
    language_converters: Vec<Box<dyn LanguageConverter + 'a>>,    
}

impl<'a> CodeConverter<'a> {
    fn new() -> Self {
        CodeConverter {
            language_converters: vec![
                Box::new(JavaConverter {}),
                Box::new(CppConverter {}),
                Box::new(KotlinConverter {}),
                Box::new(PythonConverter {}),
                Box::new(RustConverter {}),
                Box::new(GoConverter {}),
                Box::new(RubyConverter {}),
                Box::new(SwiftConverter {}),
                Box::new(CSharpConverter {}),
                // Add other converters here
            ],    
        }
    }
}
