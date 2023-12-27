use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error at line {} column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for JsonError {}

impl JsonError {
    fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
        }
    }
}

fn main() {
    let input = r#"{"name": "Alice", "age": "30"}"#;
    let parsed_result = serde_json::from_str::<serde_json::Value>(input);

    if let Err(err) = parsed_result {
        let line = err.line();
        let column = err.column();
        let message = err.to_string();
        let json_err = JsonError::new(message, line, column);

        eprintln!("Error: {}", json_err);
        std::process::exit(1);
    }
    // Rest of the program
}
