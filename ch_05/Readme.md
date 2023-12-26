# References

References in Rust are denoted with an `&` symbol and are used to borrow data from a source, rather than taking ownership of the data. This means that when a reference goes out of scope, it does not cause the data it refers to to be deallocated.

References are a way to allow multiple parts of your code to read or write to a single piece of data without requiring ownership of that data. This can be useful in many situations, such as when you want to pass a large piece of data to a function without copying it, or when you want to allow multiple parts of your code to access a single resource.

Here is an example of using a reference:

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

In this example, the `calculate_length` function takes a reference to a `String` as its argument. This reference is denoted with the `&` symbol. The function does not take ownership of the `String`, so it is still available to be used after the function call.

In this example, the `calculate_length` function takes a reference to a `String` as its argument. This reference is denoted with the `&` symbol. The function does not take ownership of the `String`, so it is still available to be used after the function call.

It is also possible to use references to mutate the data they refer to. This is done by using a mutable reference, which is denoted with the `&mut` symbol. Here is an example of using a mutable reference:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

In this example, the `change` function takes a mutable reference to a `String` as its argument. This mutable reference is denoted with the `&mut` symbol. The function is able to modify the value of the `String` through the reference.

The borrow checker in Rust enforces rules around the use of references to ensure the safety and correctness of code.

**These rules include**:

1. **Single Mutable Reference (`&mut`)**:

   ```rs
   fn main() {
       let mut data = 42;

       let reference1 = &mut data; // Mutable reference
       // let reference2 = &mut data; // Uncommenting this line would cause a compile-time error
       // Creating a second mutable reference in the same scope is disallowed

       *reference1 = 10; // Modifying the data through the mutable reference
       println!("Modified data: {}", *reference1);
   }
   ```

2. **Multiple Immutable References (`&`)**:

   ```rs
   fn main() {
       let data = 42;

       let reference1 = &data; // First immutable reference
       let reference2 = &data; // Second immutable reference

       println!("References: {:p} and {:p}", reference1, reference2);
   }
   ```

3. **No Mixing Mutable and Immutable References**:

   ```rs
   fn main() {
       let mut s = String::from("hello");

       let r1 = &s; // no problem
       let r2 = &s; // no problem
       println!("{} and {}", r1, r2);
       // variables r1 and r2 will not be used after this point

       let r3 = &mut s; // no problem
       println!("{}", r3);
   }
   ```

4. **Borrowing Duration**:

   ```rs
   fn main() {
       let reference;

       {
           let data = 42;
           reference = &data; // Reference to 'data' within this scope
       }

       // println!("Data: {}", *reference); // Uncommenting this line would cause a compile-time error
       // 'reference' would be referencing memory that is no longer valid
   }
   ```

5. **No Dangling References**:

```rs
fn dangle() -> &i32 {
    let data = 42;
    &data // Returning a reference to 'data'
          // 'data' goes out of scope here
}

fn main() {
    let reference_to_nothing = dangle(); // Uncommenting this line would cause a compile-time error
    // Attempting to create a reference to 'data' that no longer exists
}
```
