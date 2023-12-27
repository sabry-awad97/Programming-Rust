use std::error::Error;

use backtrace::Backtrace;
fn recursive_function(n: i32) -> Result<(), Box<dyn Error>> {
    if n == 0 {
        Err("error message".into())
    } else {
        recursive_function(n - 1)
    }
}

fn main() {
    let bt = Backtrace::new();
    let result = recursive_function(10);
    match result {
        Ok(_) => println!("Success!"),
        Err(e) => {
            println!("Error: {}", e);
            println!("Backtrace: {:?}", bt);
        }
    }
}
