fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "nonexistent.txt";
    let file = std::fs::File::open(file_name)?;

    Ok(())
}

fn handle_error(err: Box<dyn std::error::Error>) {
    println!("Error: {}", err);
    eprintln!("Error: {}", err);
    let error_message = format!("Error: {}", err);
    // do something with the error message
}
