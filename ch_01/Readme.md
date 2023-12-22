# Chapter 1. Systems Programmers Can Have Nice Things

"Systems programmers can have nice things" is a phrase often used to describe the benefits of using the Rust programming language for systems programming tasks. Rust is a programming language that was designed to be safe, concurrent, and fast, making it well-suited for use in systems programming contexts.

One of the key features of Rust that makes it well-suited for systems programming is its emphasis on safety. Rust uses a type system that is statically checked at compile time, which helps to prevent common programming errors such as **null** or **dangling pointer references**. This can help to reduce the number of bugs in systems-level code, which can be especially important when dealing with low-level hardware interactions or resource-constrained environments.

**Null References**: In many programming languages, null references can lead to runtime errors. However, in Rust, the type system ensures that variables cannot be null unless explicitly stated as such using the Option type.

```rs
fn main() {
    let present_value: Option<i32> = Some(42); // A value is present
    let absent_value: Option<i32> = None;     // No value present

    // Accessing the value from Option
    match present_value {
        Some(value) => println!("Value is: {}", value), // Prints: Value is: 42
        None => println!("No value!"),
    }

    // Uncommenting the line below would cause a compile-time error
    // let value = absent_value.unwrap(); // This line triggers a compile-time error (used `unwrap()` on `None` value)
}
```

In this example, `present_value` holds a value (Some(42)), while `absent_value` is explicitly set as `None`. Trying to directly access a value from `None` using `unwrap()` would result in a compile-time error, preventing potential issues with null references at runtime.
