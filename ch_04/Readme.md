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

## Moves

In Rust, a move is the act of transferring ownership of a value from one owner to another. When a value is moved, the original value is no longer valid and cannot be used.

- Moves are often used to pass values to functions or to rearrange the ownership tree

  ```rust
  fn main() {
      let x = vec![1, 2, 3];
      let y = x;

      println!("{:?}", x);
  }
  ```

- Moves are also used when returning values from functions

  ```rust
  fn create_vec() -> Vec<i32> {
      let x = vec![1, 2, 3];
      x
  }

  fn main() {
      let y = create_vec();
      println!("{:?}", y);
  }
  ```

In Rust, most types are moved rather than copied when they are assigned to a new variable, passed to a function, or returned from a function. This means that the source relinquishes ownership of the value to the destination, and the value's lifetime is now controlled by the destination

Yes, it is true that the use of moves in Rust may be surprising to some people, as assignment is a fundamental operation in programming languages that is typically well-defined. However, different programming languages handle assignment in different ways, and Rust's decision to use moves is a result of its design goals and the safety guarantees it aims to provide.

In many languages, assignment simply copies the value from the source to the destination, leaving the original value unchanged. This is known as "copy semantics." However, Rust uses "move semantics," which means that assignment transfers ownership of the value from the source to the destination, rendering the original value invalid.

One reason for this is that Rust's design goals include memory safety and the prevention of common memory safety issues such as dangling pointers and use of uninitialized memory. Using move semantics helps ensure that values are not used after they are no longer needed and that memory is properly managed.

Additionally, Rust's move semantics can make it easier to reason about the lifetime of values in your code. Since a value can only have one owner, it is clear when a value will be dropped and when its memory will be freed. This can make it easier to understand and predict the behavior of your code.

Here are some key points about moves in Rust:

- Ownership is a system that prevents data races and segmentation faults by ensuring that each piece of data is owned by exactly one variable at a time.
- Ownership is implemented through the use of "moves."
- When a value is moved, the original value is no longer available for use.
- When a value is moved, the value is transferred to a new owner without making a copy.
- When a value is moved, any references to the original value become invalid.
- When a value is moved, the ownership of the value is transferred to the new owner.
- When a value is moved, the value is moved "by value," meaning that the value itself is transferred, not just a reference to it.

Here is an example of a move:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    println!("{}, world!", s1); // error: s1 has been moved
}
```

In this example, the value of `s1` is moved to `s2`, and the original value of `s1` is no longer available for use. Any attempt to use `s1` after the move will result in a compile-time error.

## Stack and Heap

Consider the following example:

```rs
fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = s;
}
```

The first line creates a new vector `s` on the heap, containing three strings. Each of these strings is also allocated on the heap and contains the text "udon", "ramen", and "soba" respectively.

In the second line, the value of `s` is moved to `t`. This means that the value of `s` is no longer valid, and attempting to use it will result in a compile-time error. The vector `t` now owns the values that were previously owned by s.

In the third line, the value of s is used agin. This would result in a compile-time error because the value of s is no longer valid after it has been moved.

After these three lines are executed, the ownership tree in the heap looks like this:

```rust
t  ->  ["udon", "ramen", "soba"]
```

The vector t owns the strings "udon", "ramen", and "soba".

In the stack, the variables s and t are all valid and contain references to the vectors on the heap. However, the value of s has been moved to t, so attempting to use s after this point would result in a compile-time error.

Before the move, the stack and heap would be as follows:

```rust
s: +------------+
    |  Pointer   |  ---->  [Heap]
    +------------+
    |  Length    |
    +------------+
    |  Capacity  |
    +------------+

t: +------------+
    |  Pointer   |
    +------------+
    |  Length    |
    +------------+
    |  Capacity  |
    +------------+

Heap:
+-------------------+
|  "udon"           |
+-------------------+
|  "ramen"          |
+-------------------+
|  "soba"           |
+-------------------+
```

The variable `s` contains a pointer to the heap-allocated array that contains the elements of the vector, as well as the length and capacity of the array. The variable `t` is uninitialized and does not contain a valid pointer. The heap-allocated array contains the elements of the vector, each element being a string slice containing the text of the element.

After the move, the stack and heap would be as follows:

```rust
s: +------------+
    |  Pointer   |
    +------------+
    |  Length    |
    +------------+
    |  Capacity  |
    +------------+

t: +------------+
    |  Pointer   |  ---->  [Heap]
    +------------+
    |  Length    |
    +------------+
    |  Capacity  |
    +------------+

Heap:
+-------------------+
|  "udon"           |
+-------------------+
|  "ramen"          |
+-------------------+
|  "soba"           |
+-------------------+
```

The value of `s` has been moved to `t`, so the variable `s` is now uninitialized and does not contain a valid pointer. The variable `t` now contains a pointer to the heap-allocated array that was previously owned by `s`, as well as the length and capacity of the array. The values in the heap are unchanged, as the elements themselves are not moved. Only the ownership of the heap-allocated array is transferred from `s` to `t`.

To make `u` vector you must ask for a copy

```rust
let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
let t = s.clone();
let u = s.clone();
```

In this version, `s` is an immutable variable and its value is cloned into `t` and u using the clone method. This creates new values on the heap that are copies of the original value in `s`.

The ownership tree in the heap after these three lines are executed would look like this:

```rust
u  ->  ["udon", "ramen", "soba"]
s  ->  ["udon", "ramen", "soba"]
t  ->  ["udon", "ramen", "soba"]
```

In this version of the code, `s`, `t`, and `u` all own separate values on the heap, and the original value of `s` is still valid and can be used after the clones are created.

```rust
struct Person {
    name: String,
    birth: i32,
}
let mut composers = Vec::new();
composers.push(Person {
    name: "Palestrina".to_string(),
    birth: 1525,
});
```

```rust
               |  Value
---------------+------------------------------------------------
composers      |  +------------+
               |  |  Pointer   |  ---->  [Heap]
               |  +------------+
               |  |  Length    |
               |  +------------+
               |  |  Capacity  |
               |  +------------+

               |  +------------+
[Heap] ---->   |  |  Person 1  |  +------------+
               |  |            |  |  Pointer   |  ---->  [Heap]
               |  |            |  +------------+
               |  |            |  |  Length    |
               |  |            |  +------------+
               |  |            |  |  Capacity  |
               |  +------------+  +------------+

               |  +------------+  +------------+
[Heap] ---->   |  |"Palestrina"|  +------------+
               |  |            |  |  Pointer   |  ---->  [Heap]
               |  |            |  +------------+
               |  |            |  |  Length    |
               |  |            |  +------------+
               |  |            |  |  Capacity  |
               |  +------------+  +------------+

[Heap] ---->   |  |"Palestrina"|  +------------+
```

The variable `composers` is a vector that owns a heap-allocated array containing one element, which is a struct `Person`. The struct `Person` owns a heap-allocated string slice containing the text of its `name` field.

## More Operations That Move

Here are some scenarios where moves occur:

- 1. **Initialization and Assignment**:
     When a value is assigned to a variable, the prior value of that variable gets dropped.
- 2. **Transfer of Ownership**:
     Assigning a value to a variable that's already initialized moves the new value in, dropping the prior value.
- 3. **Function Arguments**:
     Passing values to functions transfers ownership of those values to the function's parameters.
- 4. **Returning from Functions**:
     Returning a value from a function moves ownership of the value to the caller.
- 5. **Building Data Structures**:
     Constructing complex data structures like tuples or structs involves moving ownership of their components.

For instance, in Rust code like:

```rs
fn main() {
    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // Here, nothing is dropped; 't' owns the original string
}
```

- When `t` takes ownership of the original string from `s`, `s` becomes uninitialized, and no drop occurs when assigning a new value to `s`.

When values are moved in Rust, it might seem inefficient at first glance, but two essential points mitigate this concern.

First, the moves in Rust apply to the value itself, typically a small metadata structure (like a three-word header) rather than the potentially larger data stored in the heap. For vectors and strings, this means the actual data (like arrays or text buffers) remains in place on the heap while ownership is transferred via the value's metadata.

```rs
fn main() {
let s1 = String::from("Hello"); // s1 owns the heap-allocated string "Hello"

let s2 = s1; // Ownership of the value proper (metadata) moves to s2

// s1 is no longer valid here because ownership was moved to s2
// println!("{}", s1); // This line won't compile

// s2 still owns the same heap-allocated string "Hello"
println!("{}", s2); // This works fine
}
```

Second, Rust's compiler optimizes moves to avoid unnecessary data movement. It understands ownership semantics and often optimizes code generation to handle values in-place, reducing unnecessary copying or relocation.

```rs
fn main() {
    let s1 = String::from("Rust"); // s1 owns the string "Rust"

    let s2 = s1; // Ownership of the metadata is moved to s2

    // No actual data movement occurs, the string content "Rust" remains in place
}
```

## Moves and Control Flow

let's consider some examples of how moves can affect control flow.

### `if` Statements

If you assign a value to a new variable inside an `if` statement, the value will be moved if the `if` condition is true.

```rust
let x = vec![10, 20, 30];
if c {
    f(x); // ... ok to move from x here
} else {
    g(x); // ... and ok to also move from x here
}
h(x); // bad: x is uninitialized here if either path uses it
```

In this code, the variable `x` is a vector containing the values `10`, `20`, and `30`. The condition `c` is used to determine which of two functions, `f` and `g`, to call.

If the condition `c` is true, then the function `f` is called and the value of `x` is moved into the function as an argument. If the condition `c` is false, then the func‐ tion `g` is called and the value of `x` is also moved into the function as an argument.

After either function is called, the value of `x` is considered uninitialized because it has been moved away and has not definitely been given a new value since. Therefore, it is not allowed to use `x` in the call to `h`.

### `while` Loops

If you assign a value to a new variable inside a `while` loop, the value will be moved on each iteration of the loop.

```rust
let x = vec![10, 20, 30];
while f() {
    g(x); // bad: x would be moved in first iteration,
            // uninitialized in second
}
```

In this code, the variable `x` is a vector containing the values `10`, `20`, and `30`. The `while` loop will execute as long as the function `f` returns true.

If the function `f` returns true, then the function `g` is called and the value of `x` is moved into the function as an argument. However, after the first iteration of the loop, the value of `x` is considered uninitialized because it has been moved away and has not definitely been given a new value since. Therefore, it is not allowed to use `x` in subsequent iterations of the loop.

To avoid this error:

- You can use the clone method, which creates a deep copy of the value:

  ```rust
  fn main() {
      let x = vec![10, 20, 30];

      while f() {
          g(x.clone());
      }
  }
  ```

- Alternatively, you can declare x as a mutable variable and reassign it a new value before each iteration of the loop:

  ```rust
  fn main() {
      let mut x = vec![10, 20, 30];

      while f() {
          g(x); // move from x
          x = h(); // give x a fresh value
      }
  }
  ```

### `for` Loops

If you assign a value to a new variable inside a `for` loop, the value will be moved on each iteration of the loop.

```rust
fn main() {
    let v = vec!["A".to_string(), "B".to_string()];
    for i in v {
        let j = i; // i is moved to j on each iteration
        println!("{}", i); // error: i has been moved
    }
}
```

In this example, the value of `i` is moved to `j` on each iteration of the loop, and the original value of `i` is no longer available for use.

## Moves and Indexed Content

let's consider some examples of how moves can affect indexed content.

### Vectors

If you assign a value from a vector to a new variable, the value will be moved.

```rust
fn main() {
    let v = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let x = v[0]; // cannot move out of index of Vec
}
```

You cannot move elements out of a vector using indexing because vectors store their elements on the heap, and moving elements out of the vector would invalidate the vector's ownership of the elements.

```rs
fn main() {
    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    println!("{:?}", v); // error: borrow of moved value: `v`
}
```

The for loop's internal machinery takes ownership of the vector and dissects it into its elements. At each iteration, the loop moves another element to the variable s. Since s now owns the string, we’re able to modify it in the loop body before printing it.

### structs

```rs
struct Person {
    name: Option<String>,
    birth: i32,
}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    let first_name = composers[0].name;
}
```

You cannot move the `name` field out of the `Person` struct using indexing, because the `Person` struct is stored on the heap, and moving the field would invalidate the struct's ownership of the field.

To access the `name` field of the `Person` struct, you have a few options:

#### 1\. Borrow the field

You can borrow the `name` field of the `Person` struct using an immutable reference:

```rust
let first_name = &composers[0].name; // first_name is a &Option<String>
```

In this example, the `first_name` variable is a reference to the `name` field of the `Person` struct, and the original field is not moved.

#### 2\. Clone the field

You can clone the `name` field of the `Person` struct using the `clone` method:

```rust
let first_name = composers[0].name.clone(); // first_name is a Option<String>
```

In this example, the `first_name` variable is a copy of the `name` field of the `Person` struct, and the original field is not moved.

#### 3\. Use a reference and take ownership

You can use the `as_ref` method:

```rust
let first_name = composers[0].name.as_ref(); // first_name is a Option<&String>
```

#### 4\. Use `std::mem::replace`

Yes, you can use the `std::mem::replace` function to move the `name` field out of the `Person` struct and set the field to a new value.

```rust
let first_name = std::mem::replace(&mut composers[0].name, None); // first_name is a Option<String>
println!("{:?}", composers[0].name); // composers[0].name is None
```

In this example, the `std::mem::replace` function moves the `name` field out of the `Person` struct and sets the field to a new value of `None`. The original value of the `name` field is returned as an `Option<String>`, which can be `Some` if the field was previously set, or `None` if the field was already `None`.

#### 4\. Use `take()` method

Yes, you can use the `take` method to move the `name` field out of the `Person` struct and set the field to `None`.

Here's an example of how to use the `take` method:

```rust
fn main() {
    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()),
    birth: 1525 });
    let first_name = composers[0].name.take(); // first_name is a Option<String>
    println!("{:?}", composers[0].name); // composers[0].name is None
}
```

In this example, the `take` method moves the `name` field out of the `Person` struct and sets the field to `None`. The original value of the `name` field is returned as an `Option<String>`, which can be `Some` if the field was previously set, or `None` if the field was already `None`.

## Copy Types: The Exception to Moves

In Rust, values of certain types are "copyable", meaning that they can be assigned or passed by value without moving the original value. These types are called "copy types".

Copy types are implemented using the `Copy` trait, which is automatically implemented for certain types, such as integers, floating-point numbers, booleans, and character types.

Here's an example of using a copy type:

```rust
fn main() {
    let x = 5; // x is an i32
    let y = x; // y is also an i32
    println!("x = {}, y = {}", x, y);
}
```

In this example, the value of `x` is copied into `y` when `y` is assigned, and the original value of `x` is not moved. Both variables are independent copies of the same value.

It's worth noting that copy types are not moved when they are passed as function arguments or returned as function results. Instead, the values are copied.

Here is a list of some common copy types in Rust:

1. Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
1. Floating-point numbers: `f32`, `f64`
1. Booleans: `bool`
1. Character types: `char`
1. Tuples containing only copy types

It's worth noting that arrays and slices of copy types are also copy types, as long as the array or slice is not mutable.

```rust
fn main() {
    let a = [1, 2, 3]; // a is an array of i32
    let b = a; // b is also an array of i32
    println!("a = {:?}, b = {:?}", a, b);
}
```

The value of `a` is copied into `b` when `b` is assigned, and the original value of `a` is not moved. Both variables are independent copies of the same array.

If a struct contains fields that are copy types, then the entire struct can be stored on the stack. For example:

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
```

In this example, the `Point` struct contains two `i32` fields, which are copy types. The entire `Point` struct can be stored on the stack.

If a struct contains fields that are non-copy types, then the struct may need to store a pointer to the heap-allocated data on the stack. For example:

```rust
struct Person {
    name: String,
    age: i32,
}

let p = Person { name: "Alice".to_string(), age: 30 };
```

In this example, the `Person` struct contains a `String` field and an `i32` field. The `String` field is a non-copy type, so the `Person` struct stores a pointer to the `String` on the heap on the stack. The `i32` field is a copy type, so it is stored directly on the stack.

Structs can also contain fields that are pointers to values on the heap, such as `Box<T>` or `Rc<T>`. In these cases, the struct will store the pointer on the stack, and the pointed-to value will be stored on the heap.

## Copy Trait

In Rust, the `Copy` trait is a special trait that indicates that a type can be safely copied by simply copying its bits. This means that when you assign a value of a `Copy` type to another variable, or pass it as an argument to a function, the value is copied rather than moved.

However, not all types in Rust are `Copy`. In fact, many types are not `Copy` because they own resources that need to be cleaned up when the value is no longer used. For example, the `String` type owns a heap-allocated buffer that holds the string data. If a `String` value were simply copied by bit-for-bit duplication, it would be unclear which value was now responsible for cleaning up the heap-allocated buffer. This is why `String` is not `Copy`.

Similarly, the `Box<T>` type owns its heap-allocated referent, and duplicating it by bit-for-bit copying would leave it unclear which value was responsible for the referent. The `File` type, which represents a reference to an operating system file descriptor, is not `Copy` because duplicating it would entail asking the operating system for another file handle. The `MutexGuard` type, which represents a locked mutex, is not `Copy` because it is not meaningful to copy a value of this type at all, as only one thread may hold a mutex at a time.

As a rule of thumb, any type that needs to do something special when a value is dropped (e.g. free its elements, close a file handle, unlock a mutex) cannot be `Copy`, because it is unclear which value would be responsible for the original's resources after a bit-for-bit duplication.

To summarize, types that are not `Copy` in Rust include:

- `String`, because it owns a heap-allocated buffer
- `Box<T>`, because it owns its heap-allocated referent
- `File`, because it represents a reference to an operating system file descriptor
- `MutexGuard`, because it represents a locked mutex

Here is an example of how to use the `Copy` trait in Rust:

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 is copied to p2 here
    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
}
```

In this example, we have defined a `Point` struct with two fields: `x` and `y`. We have also added the `#[derive(Copy, Clone)]` attribute to the struct definition, which automatically implements the `Copy` and `Clone` traits for the `Point` type. This means that we can use the `Copy` trait to copy a `Point` value by simply assigning it to another variable.

Here is a summarization of Copy trait:

- The `Copy` trait in Rust indicates that a type can be safely copied by bit-for-bit duplication.
- Types that own resources that need to be cleaned up when the value is no longer used, such as heap-allocated memory or file handles, cannot be `Copy`.
- Making a type `Copy` represents a serious commitment on the part of the implementer, as it limits the types that the type can contain and restricts the ways in which the type can be used.
- In contrast to C++, Rust does not permit the customization of copy and move operations.
- Every move in Rust is a byte-for-byte, shallow copy that leaves the source uninitialized.
- This means that basic operations like assignment, passing parameters, and returning values from functions are more predictable in Rust, as the costs of these operations are more explicit and apparent to the programmer.
- In C++, assigning one variable to another can require arbitrary amounts of memory and processor time.
- Rust's approach makes basic operations simple and potentially expensive operations explicit, like calls to `clone` that make deep copies of vectors and their contents.

## Rc and Arc: Shared Ownership

`Rc` (short for "reference counted") and `Arc` (short for "atomic reference counted") are types that provide shared ownership of a value. This means that multiple `Rc` or `Arc` values can point to the same value, and the value will be dropped only when the last `Rc` or `Arc` value pointing to it is dropped.

### `Rc` (Reference Counting)

- **Usage in Single-threaded Contexts:**

  - Suitable for scenarios where shared ownership is required within a single thread.
  - Allows multiple immutable references to the same data.
  - Increases a reference count each time a new reference is created.
  - Decreases the count when a reference goes out of scope within the same thread.

  ```rs
  use std::rc::Rc;

  fn main() {
      let s1 = Rc::new("Hello".to_string());
      let s2 = s1.clone();
      let s3 = s1.clone();

      println!("{} {} {}", s1, s2, s3);

      // Get the current reference count
      let count = Rc::strong_count(&s1);
      println!("Reference count of s1: {}", count);
  }
  ```

### `Arc` (Atomic Reference Counting)

- **Usage in Multi-threaded Contexts:**

  - Used in scenarios where shared ownership is needed across multiple threads safely.
  - Similar to `Rc` but implements atomic operations for thread safety.
  - Allows multiple threads to share and access data concurrently.
  - Ensures synchronized access to shared data by using atomic reference counting.

  ```rs
  use std::sync::Arc;

  fn main() {
      let s1 = Arc::new("Hello".to_string());
      let s2 = s1.clone();
      let s3 = s1.clone();

      println!("{} {} {}", s1, s2, s3);

      // Get the current reference count
      let count = Arc::strong_count(&s1);
      println!("Reference count of s1: {}", count);
  }
  ```

### `Drop` Trait and Ownership

- **`Drop` Trait Implementation:**
  - They track the reference count and deallocate the shared data when the count reaches zero, indicating no active references exist.

### Synchronization Mechanisms

- **Usage of Mutexes**

  - To ensure safe concurrent access to shared data when using `Arc`, synchronization mechanisms like mutexes should be employed.
  - Mutexes help in controlling access to shared data by allowing only one thread to access the data at a time.

  - Rust provides the `std::sync::Mutex` type, which allows safe mutable access to shared data across threads by acquiring and releasing locks.

  ```rs
  use std::sync::{Arc, Mutex};

  fn main() {
      // Creating shared data wrapped in an Arc and a Mutex
      let shared_data = Arc::new(Mutex::new(0));

      // Cloning Arc to create multiple references for different threads
      let shared_data_clone1 = Arc::clone(&shared_data);
      let shared_data_clone2 = Arc::clone(&shared_data);

      // Spawn threads to concurrently modify the shared data
      let thread1 = std::thread::spawn(move || {
          let mut data = shared_data_clone1.lock().unwrap();
          *data += 1; // Modifying the shared data
      });

      let thread2 = std::thread::spawn(move || {
          let mut data = shared_data_clone2.lock().unwrap();
          *data += 2; // Modifying the shared data
      });

      // Waiting for threads to complete execution
      thread1.join().unwrap();
      thread2.join().unwrap();

      // Accessing the modified shared data
      let final_data = shared_data.lock().unwrap();
      println!("Final value: {}", *final_data);

      // Get the current reference count
      let count = Arc::strong_count(&shared_data);
      println!("Reference count of shared_data: {}", count);
  }
  ```

Here is a summary of the main points of interest:

|                              | `Rc`                      | `Arc`                                                |
| ---------------------------- | ------------------------- | ---------------------------------------------------- |
| Meaning                      | Reference counted         | Atomic reference counted                             |
| Use cases                    | Single-threaded           | Multi-threaded                                       |
| Implements `Deref` trait     | Yes                       | Yes                                                  |
| Overhead                     | Reference count           | Reference count + atomic operations                  |
| Concurrent access protection | None                      | Yes                                                  |
| Common use cases             | Tree-like data structures | Tree-like data structures in concurrent environments |

### Reference Cycle

Reference cycles occur when two or more values have references to each other, creating a circular reference. Because each value holds a reference to the other, the reference count for each value will never reach zero, and the values will never be deallocated from memory. This can lead to a memory leak, where the values continue to occupy memory even though they are no longer needed.

The `Rc` pointers in Rust are immutable, which means creating a direct cyclic reference between two `Rc` pointers isn't possible because it requires mutability of the reference, and `Rc` doesn't allow this.

When dealing with potential cyclic references using Rc pointers, you can break those cycles by using `std::rc::Weak` pointers for some of the links. This strategy involves using Weak pointers to prevent strong ownership and thus breaking the cycle, allowing memory to be properly deallocated.
