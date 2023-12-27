use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Custom Error: {}", self.message)
    }
}

fn main() -> Result<(), CustomError> {
    let err = CustomError {
        message: "Something went wrong".to_string(),
    };
    Err(err)
}
