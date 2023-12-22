fn main() {
    let present_value: Option<i32> = Some(42); // A value is present
    let absent_value: Option<i32> = None; // No value present

    // Accessing the value from Option
    match present_value {
        Some(value) => println!("Value is: {}", value), // Prints: Value is: 42
        None => println!("No value!"),
    }

    // Uncommenting the line below would cause a compile-time error
    // let value = absent_value.unwrap(); // This line triggers a compile-time error (used `unwrap()` on `None` value)
}
