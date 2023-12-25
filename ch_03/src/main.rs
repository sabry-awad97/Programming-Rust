type Result<T> = std::result::Result<T, String>;

fn read_file(file_name: &str) -> Result<String> {
    let contents = std::fs::read_to_string(file_name).map_err(|e| e.to_string())?;
    Ok(contents)
}

fn main() {
    let contents = read_file("file.txt").unwrap();
    println!("File contents: {}", contents);
}
