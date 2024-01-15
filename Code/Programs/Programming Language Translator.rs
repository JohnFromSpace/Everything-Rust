use std::fmt::Write;

#[derive(Debug)]
struct RustFunction {
    name: String,
    parameters: Vec<(String, String)>,
    return_type: String,
    body: String,
}
