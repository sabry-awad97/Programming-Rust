# Ownership and Moves

At the core of Rust's ownership system are three key principles: ownership, borrowing, and lifetimes.

Ownership is based on the idea that each value in Rust has a single owner, and the owner is responsible for the value's lifetime. The owner is responsible for deallocating the memory when the data is no longer needed.

One aspect of ownership is "move semantics". When a value is assigned to a new variable or passed to a function, the ownership of the value is transferred to the new variable. This is known as a "move". For example:

```rust
fn main() {
    let original = String::from("Hello"); // String allocated on the heap
    let new_var = original; // Ownership of the String moves to 'new_var'

    // Trying to access 'original' here would result in a compile-time error
    // because the ownership has been moved to 'new_var'
    // println!("{}", original); // This line won't compile

    // 'new_var' still owns the String data and can be used
    println!("{}", new_var); // This works fine
}
```

In this code, the value of `original` is moved to `new_var`, and `original` is no longer valid. This is because Rust assumes that the original owner is no longer interested in the value and it is safe to transfer ownership to the new owner (`new_var`).

The `original` variable is no longer valid to prevent multiple pointers to the same data (ensuring memory safety).

Moves help prevent data races by ensuring that only one owner has access to and can modify the data at a time.

Move semantics are important because they allow Rust to optimize memory usage by avoiding unnecessary copies of data. However, they also mean that once a value has been moved, the original value can no longer be used.

You can use the `clone` method to create a deep copy of a value if you need to use the original value after it has been moved. This can be useful in cases where you need to retain ownership of a value while also using it in multiple places
