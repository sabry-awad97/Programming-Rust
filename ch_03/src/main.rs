fn main() {
    let greeting = "Hello";
    let number = 42;
    let width = 10;

    // Using variables within padding format
    let aligned_greeting = format!("{:<width$}World", greeting, width = width);
    let aligned_number = format!("{:>width$}", number, width = width);
    let centered_text = format!("{:^width$}", "Rust", width = width);
    let padded_number = format!("{:*>width$}", number, width = width);

    // Printing formatted output
    println!("Left Aligned: {}", aligned_greeting);
    println!("Right Aligned: {}", aligned_number);
    println!("Center Aligned: {}", centered_text);
    println!("Padded with *: {}", padded_number);
}
