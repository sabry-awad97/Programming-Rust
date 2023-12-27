use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Custom error: {}", self.message)
    }
}

impl Error for CustomError {}

fn do_something() -> Result<(), Box<dyn Error>> {
    let result = do_something_else()?;
    Ok(result)
}

fn do_something_else() -> Result<(), Box<dyn Error>> {
    Err(Box::new(CustomError {
        message: "oops".to_string(),
    }))
}

fn main() {
    let result = do_something();
    if let Err(err) = result {
        println!("Error: {}", err);
        if let Some(custom_error) = err.downcast_ref::<CustomError>() {
            println!("Custom error message: {}", custom_error.message);
        }
    }
}
