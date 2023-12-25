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

## Safety First vs Control First

When it comes to **managing memory**, there are two characteristics we’d like from our programing languages:

- We'd like memory to be freed promptly, at a time of our choosing. This gives us **control** over the program's memory consumption.
- We never want to use a pointer to an object after it's been freed (_dangling pointer_). This would be undefined behavior, leading to crashes and security holes.

  - A **dangling pointer** is a pointer that points to memory that has already been deallocated (freed).

    For example, consider the following code in a language that does not have garbage collection:

    ```c++
    int *p = malloc(sizeof(int));
    *p = 42;
    free(p);
    printf("%d\n", *p);
    ```

    In this code, the pointer `p` is initially pointing to a block of memory that has been allocated with `malloc`. The value `42` is then written to this memory location. However, the memory is then freed with `free`, which invalidates the pointer.

    The final line of code then attempts to dereference the pointer `p`, which is a dangling pointer at this point. This can lead to undefined behavior, such as a program crash or incorrect output.

These two characteristics of memory management can be mutually exclusive, and many programming languages choose to prioritize one over the other.

- Languages in the **"Safety First"** camp, such as Python, Javascript, Ruby, and Java, use garbage collection to _**automatically manage memory**_ and _**avoid dangling pointers**_. This can make it easier to write correct programs, but it also means that you have less control over when memory is freed and may need to rely on the garbage collector to determine when objects are no longer needed.

- Languages in the **"Control First"** camp, such as C and C++, give the programmer _**more control**_ over memory management but also _**require more care to avoid dangling pointers**_. This can be more challenging, but it also gives the programmer more control over the program's memory usage and can lead to more efficient programs.

Rust is a unique language in that it tries to combine the best of both worlds by offering both _**safe memory management**_ and _**control over memory usage**_. It does this through its ownership and borrowing system, which provides automatic memory management while also giving the programmer control over the lifetime of values and the ability to optimize memory usage.

Here is a summary of the key differences between the "Safety First" and "Control First" approaches to memory management, as well as how Rust handles these issues:

|                                | Safety First | Control First | Rust                               |
| ------------------------------ | ------------ | ------------- | ---------------------------------- |
| Automatic memory management    | Yes          | No            | **Yes** (through ownership system) |
| Control over memory usage      | No           | Yes           | **Yes** (through borrowing system) |
| Avoidance of dangling pointers | Yes          | No            | **Yes** (through borrowing system) |
| Predictability of memory usage | Yes          | No            | **Yes** (through ownership system) |
