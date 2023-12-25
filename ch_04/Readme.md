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

## Examples of ownership system

```rust
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped her
```

In this example, the variable `padovan` is a vector that is allocated on the heap. It is owned by the `print_padovan` function, so when the function finishes executing and control leaves the block in which padovan is declared, `padovan` is dropped and its memory is freed.

The ownership system ensures that values are not used after they are no longer needed, which can help prevent memory leaks and make it easier to reason about the lifetime of values in a Rust program.

```rust
fn main () {
    let point = Box::new((0.625, 0.5)); // point allocated here
    let label = format!("{:?}", point); // label allocated here
    assert_eq!(label, "(0.625, 0.5)");
} // both dropped here
```

In this example, the `Box` type is used to allocate a tuple of two `f64` values on the heap. The `Box::new` function takes a tuple as an argument and returns a `Box` pointing to the heap space where the tuple is stored.

Since the `Box` owns the heap space it points to, when the `Box` is dropped, it frees the space as well. In this example, the `point` variable is a `Box` pointing to the heap-allocated tuple, and the `label` variable is a string containing a formatted version of the tuple.

When the program finishes executing and control reaches the closing curly brace, both the `point` and `label` variables are dropped. This frees the heap-allocated tuple and the memory used for the `label` string.

Overall, the `Box` type is a useful tool for allocating values on the heap in Rust and managing their lifetimes. It allows you to store values on the heap and automatically free the memory when the `Box` is no longer needed.

```rust
fn main() {
    struct Person {
        name: String,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}
```

In this example, a struct called `Person` is defined with two fields: `name` and `birth`. The `name` field is a `String` and the `birth` field is an `i32`.

The `composers` variable is a `Vec` of `Person` structs, which is created using the `Vec::new` function. Three `Person` structs are then pushed onto the `composers` vector using the `push` method.

The `for` loop then iterates over the `composers` vector using a reference `&composer`. The `println!` macro is used to print the `name` and `birth` fields of each `composer`.

This code demonstrates how structs own their fields and how vectors own their elements. When the `composers` vector is dropped at the end of the program, the `Person` structs it contains are also dropped, along with their `name` and `birth` fields. This frees the memory used by the `Person` structs and their fields.

This cascading effect of dropping values is an important aspect of the ownership system in Rust. It helps ensure that all values are properly cleaned up and that memory is freed when it is no longer needed which can help prevent memory leaks and make it easier to reason about the lifetime of values in a Rust program.

It is also important to keep in mind that when a value is moved to a new owner, the original value is no longer valid and cannot be used. This is why, the `push` method is used to add new elements to the `composers` vector, rather than directly assigning them to elements of the vector. If a value is moved, any attempts to use it will result in a compile-time error.

The ownership relationships between values in a Rust program can be thought of as a tree, with each value having a single owner and the values it owns being its children. The root of the tree is a variable, and when that variable goes out of scope, the entire tree is dropped.

To remove a value from the ownership tree in Rust, you can either move it to a new owner or delete it from a data structure. For example, you can move a value to a new owner by assigning it to a new variable or passing it as an argument to a function. You can delete a value from a data structure by using methods like `pop` or `remove` or by using the `Drop` trait to define custom behavior for dropping values.

When a value is removed from the ownership tree in this way, Rust ensures that it is properly dropped, along with everything it owns.

While the basic concept of ownership in Rust is simple, the language extends it in several ways to make it more flexible and practical to use.

Some of the ways that Rust extends the concept of ownership include:

- Moving values: In Rust, you can move a value from one owner to another, which allows you to build, rearrange, and tear down the ownership tree as needed. This is often done using the `std::mem::swap` or `std::mem::replace` functions, or by simply assigning the value to a new variable.
- **Copy types**: Some simple types in Rust, such as integers, floating-point numbers, and characters, are marked as `Copy`. This means that when a value of one of these types is moved, the original value is not dropped and is still usable. This allows you to make copies of these types without having to worry about ownership issues.
- **Reference-counted pointers**: The `std::rc::Rc` and `std::sync::Arc` types in the standard library allow values to have multiple owners by keeping track of the number of references to the value. This can be useful in certain situations, but it comes with some restrictions to ensure that the ownership rules are not violated.
- **References**: In Rust, you can create a reference to a value, which is a non-owning pointer to the value with a limited lifetime. References allow you to access a value without taking ownership of it, and they are often used to pass values to functions or to work with data structures that do not have a single owner.