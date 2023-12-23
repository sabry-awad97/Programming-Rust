use std::env;

fn main() {
    // Print the command line arguments
    for arg in env::args() {
        println!("{}", arg);
    }

    // Get the value of the "PATH" environment variable
    let path = env::var("PATH").unwrap_or_else(|e| {
        eprintln!("Error getting PATH: {}", e);
        std::process::exit(1);
    });
    println!("PATH: {}", path);
}
