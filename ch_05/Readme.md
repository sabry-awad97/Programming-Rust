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

Here is a list of terms that are commonly used when discussing references in Rust:

- **Reference**: A non-owning pointer type that allows you to borrow data from a source without taking ownership of it. Denoted with the `&` symbol.
- **Mutable reference**: A reference that allows you to modify the data it refers to. Denoted with the `&mut` symbol.
- **Referent**: The data that is being referred to by a reference.
- **Borrow**: The act of using a reference to borrow data from a source without taking ownership of it.
- **Borrow checker**: A system in Rust that enforces rules around the use of references in order to ensure the safety and correctness of code.
- **Dangling reference**: A reference to data that no longer has a clear owner. This can cause undefined behavior.

```rust
fn main() {
    let s = String::from("hello"); // s is a referent
    let r = &s; // Borrow s with a reference
    let t = &r; // t is also a reference to s
    println!("{}", t);
}
```

In this example, we borrow the data in `s` with a reference `r`, and then create another reference `t` to `r`. Both `r` and `t` are references to the same referent, `s`.

To emphasize the idea that a reference is borrowing data from a source, Rust refers to creating a reference as borrowing the value. This helps to emphasize the fact that the reference does not own the data, and that the data must eventually be returned to its owner.

In Rust, references must never outlive their referents. This means that the lifetime of a reference must be strictly shorter than the lifetime of the data it refers to. This means that a reference cannot be used after the data it refers to has been deallocated or goes out of scope.

Here is an example of a situation where a reference could potentially outlive its referent:

```rust
fn main() {
    let s = String::from("hello");
    let r = &s; // Borrow s with a reference
    let t = r; // t is also a reference to s
    drop(s); // s goes out of scope and is deallocated
    println!("{}", t); // Error: t is a dangling reference
}
```

In this example, we create a `String` called `s` and borrow it with a reference `r`. We then create another reference `t` to `r`. However, when we call `drop` on `s`, it goes out of scope and is deallocated. This means that `t` becomes a dangling reference, because it is still pointing to data that no longer exists. If we tried to use `t` after `s` has been deallocated, it would cause undefined behavior.

The distinction between shared and mutable references is a way to enforce a multiple readers or single writer rule at compile time.

A multiple readers or single writer rule is a way to control concurrent access to a shared resource. It specifies that either multiple readers can access the resource concurrently, or a single writer can access the resource exclusively.

It is important to carefully consider whether to pass a value by value or by reference, depending on whether you want the function to take ownership of the value or just access it.

Rust references cannot be null, which means that they always refer to a valid value.

In Rust, the `.` operator can also implicitly borrow a reference to its left operand if needed for a method call. For example, you can call the `sort` method on a vector like this:

```rust
let mut v = vec![1973, 1968];
v.sort(); // implicitly borrows a mutable reference to v
```

This is equivalent to calling `sort` with a more explicit, verbose syntax:

```rust
let mut v = vec![1973, 1968];
(&mut v).sort(); // equivalent, but more verbose
```

## Assigning References

In Rust, assigning a reference to a variable makes the variable point to a new location in memory. This is different from C++ references, which do not allow you to change where they point after they have been initialized.

For example, in Rust you can do the following:

```rust
let x = 10;
let y = 20;
let mut r = &x;
if true { r = &y; }
assert!(*r == 10 || *r == 20);
```

Here, the reference `r` initially points to `x`. If `b` is true, the code points it at `y` instead. This means that `r` can now point to either `x` or `y`, depending on the value of `b`.

In C++, assigning a value to a reference stores the value in its referent. Once a C++ reference has been initialized, there is no way to make it point to anything else. For example:

```rust
int x = 10;
int y = 20;
int &r = x;
if (true) { r = y; } // stores y in x, r still points to x
```

In this C++ code, the reference `r` is initialized to point to `x`. If `b` is true, the value of `y` is stored in `x`, but `r` still points to `x`. There is no way to make `r` point to `y`.

## References to References

It is possible to create references to references in Rust, which are known as "double references" or "reference of reference" types. These types can be useful in certain scenarios, such as when working with raw pointers or when writing generic code that needs to accept a variety of different reference types.

Here is an example of creating a double reference in Rust:

```rust
let x = 10;
let r1 = &x; // r1 is a shared reference to x
let r2 = &r1; // r2 is a shared reference to r1, which is a reference to x
```

You can dereference a double reference with the `**` operator:

```rust
let x = 10;
let r1 = &x;
let r2 = &r1;
let y = **r2; // y is now 10
```

It is important to note that creating double references can be dangerous, because it can be easy to lose track of how many levels of indirection are involved. This can lead to bugs, especially when working with raw pointers, where the type system does not provide any safety guarantees.

## Comparing References

In Rust, you can compare references with the `==` and `!=` operators, just like you would with other types. For example:

```rust
let x = 10;
let y = 20;
let r1 = &x;
let r2 = &y;

assert!(r1 != r2); // r1 and r2 point to different values
```

When you compare references, Rust compares the values that the references point to, rather than the references themselves. This means that two references are equal if and only if they point to the same value.

It is important to note that you can only compare references of the same type. For example, you cannot compare a shared reference to an integer with a mutable reference to a string:

```rust
let x = 10;
let mut y = "hello";
let r1 = &x;
let r2 = &mut y;

assert!(r1 != r2); // cannot compare references of different types
```

If you want to compare the memory addresses of two references, you can use the `std::ptr::eq` function. This function takes two references as arguments and returns `true` if they point to the same memory address, and `false` otherwise.

Here is an example:

```rust
use std::ptr;

fn compare_memory_addresses(x: &i32, y: &i32) -> bool {
    ptr::eq(x, y)
}

fn main() {
    let x = 10;
    let y = 20;
    let r = &x;

    assert!(!compare_memory_addresses(&x, &y)); // x and y are stored at different memory addresses
    assert!(compare_memory_addresses(r, &x)); // r and &x point to the same memory address
}
```

It is important to note that the `std::ptr::eq` function is not intended for general use, and should only be used in cases where you need to compare the memory addresses of references. In most cases, you should use the `==` operator to compare the values that the references point to, rather than the references themselves.

## References Are Never Null

A null pointer is a special value that is used to indicate that a pointer does not currently refer to a valid memory address. In some programming languages, such as C and C++, a null pointer is represented by a special value called `NULL` or `nullptr`, which is typically defined as a constant with a value of zero.

Null pointers are often used as a sentinel value to indicate the absence of a value, or as a placeholder for a pointer that has not yet been initialized. However, null pointers can also be the source of serious bugs and security vulnerabilities, as they can be dereferenced (accessed through the pointer) by mistake, leading to undefined behavior and potentially causing a crash or allowing attackers to exploit the program.

Instead of using null pointers to indicate the absence of a value, Rust has the `Option<T>` type, which can either be `Some(T)` if a value is present, or `None` if the value is absent and and it uses explicit lifetime annotations to specify the scope in which references are valid. These features help prevent null reference errors and enable Rust to provide strong guarantees about memory safety and data ownership.

For example, consider the following code:

```rust
fn find_first_even(numbers: &[i32]) -> Option<&i32> {
    for number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}

let numbers = [1, 3, 5, 7, 9];
let result = find_first_even(&numbers);

match result {
    Some(n) => println!("The first even number is {}", n),
    None => println!("There are no even numbers in the list"),
}
```

In this example, the function `find_first_even` takes a slice of integers and returns an `Option<&i32>`, which is either `Some(&i32)` if an even number is found, or `None` if no even numbers are found.

Using the `Option<T>` type instead of null pointers allows Rust to ensure that you always check for the presence of a value before using it, which helps prevent null reference errors.

It is also worth noting that Rust has a number of other features that help prevent null reference errors, such as the `NonNull<T>` type, which is a reference-like type that is guaranteed to be non-null, and the `MaybeUninit<T>` type, which allows you to temporarily store uninitialized values. These types can be used in conjunction with the `unsafe` keyword to provide more fine-grained control over null references in Rust. However, it is generally recommended to use Rust's built-in borrowing and ownership system and explicit lifetime annotations to avoid null reference errors, as these features are safer and easier to use in most cases.

## Borrowing References to Arbitrary Expressions

In Rust, you can borrow a reference to any expression, not just variables. The reference will be valid for the lifetime of the expression. For example, you can borrow a reference to a constant value, like this:

```rust
let x = 10;
let y = 20;

let r1 = &(x + y); // r1: &i32
assert_eq!(*r1, 30);
```

Here, `r1` is a shared reference to the result of the `x + y` expression, which is a constant value of `30`.

You can also borrow a reference to a more complex expression, like this:

```rust
let v = vec![1, 2, 3];

let r2 = &v[1..]; // r2: &[i32]
assert_eq!(*r2, [2, 3]);
```

Here, `r2` is a shared reference to a slice of the `v` vector, which includes the elements at indices 1 and 2.

```rust
fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn main() {
    let r = &factorial(6);
    // Arithmetic operators can see through one level of references.
    assert_eq!(r + &1009, 1729);
}
```

The `+` operator is able to see through one level of references, so the addition is performed on the values pointed to by `r` and `&1009`. If this assertion is true, the code will continue to execute. If it is false, the program will panic.

Borrowing a reference to an expression can be useful when you want to pass the result of the expression to a function or method that expects a reference, without creating a new variable to store the result. However, it is important to keep in mind that the reference will only be valid for the lifetime of the expression, and you cannot use the reference after the expression goes out of scope.

## References to Slices and Trait Objects

### Slices

Slices are a view into a contiguous sequence of elements in memory, and they are represented by the type `&[T]`. For example, a slice of integers is written as `&[i32]` and a slice of strings is written as `&[String]`. Slices allow you to borrow a portion of an array or vector, rather than the entire thing.

```rust
fn print_slice(slice: &[i32]) {
    for item in slice {
        println!("{}", item);
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // creates a slice of the array from index 1 to 3 (excluding)

    print_slice(slice); // prints "2 3"
}
```

### Traits and Trait Objects

Trait object allow for working with multiple types through a common interface. They enable polymorphism by allowing different concrete types to be treated as a single type, making code more flexible and reusable. Trait objects are useful when you need to **store a value of unknown type** in a struct or when you want to **call a method on a value without knowing its exact type** (dynamic dispatch).

- **Traits:**
  - Traits define behavior or functionality that types can implement.
  - They specify a set of methods or behavior that types must provide.
- **Trait Objects:**
  - Trait objects allow treating different types as a single type if they implement the same trait.
  - They are created by using the `dyn` keyword with a trait name, indicating a reference or box to an unknown type implementing that trait.
  - Trait objects can only be used for traits that have `object safety`, meaning they don't include associated types.

Usage of Trait Objects:

- **Dynamic Dispatch:**
  - Trait objects enable dynamic dispatch, allowing the selection of appropriate method implementations at runtime.
  - This facilitates working with different concrete types through a common trait interface.

```rs
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side_length: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side_length: 4.0 };

    print_area(&circle as &dyn Shape); // Using Circle as a Shape trait object
    print_area(&square as &dyn Shape); // Using Square as a Shape trait object
}
```

In this example, `Shape` is a trait defining the `area()` method. `Circle` and `Square` are structs implementing the `Shape` trait. The `print_area()` function accepts trait objects (`&dyn Shape`) as arguments, allowing it to work with different types that implement the `Shape` trait.

Trait objects in Rust provide a way to achieve polymorphism and abstract over different types, allowing more generic and reusable code across various concrete types that implement the same trait.

Both slices and trait objects are implemented using **fat pointers**, which are references that include both a pointer to the **data** and a pointer to **metadata**. The metadata includes information about the length of the slice or the type of the value being pointed to. This additional information allows slices and trait objects to be used safely and efficiently.

### Implementing in Programming

The term "implement" in programming refers to the act of providing the necessary code that fulfills the requirements or expectations defined by an interface, specification, or contract.

- **Interface Fulfillment:**
  - Implementing typically involves writing code for functions, methods, or behavior specified by an interface, trait, or abstract definition.
- **Meeting Requirements:**
  - It means creating functionality that adheres to the defined structure, behavior, or functionality expected by a specific interface, allowing an object or type to fulfill that contract.

In Rust, when a struct implements a trait, it means that the struct provides the necessary methods or behavior defined by that trait.

```rs
trait Printable {
    fn print(&self);
}

struct Message {
    content: String,
}

impl Printable for Message {
    fn print(&self) {
        println!("{}", self.content);
    }
}
```

In this example, the `Printable` trait defines a `print()` method. The `Message` struct implements the `Printable` trait by providing an implementation for the `print()` method. This implementation fulfills the contract specified by the `Printable` trait, allowing instances of `Message` to be treated as `Printable` and use the `print()` method.

### Fat Pointers

A "fat pointer" refers to a special kind of pointer that includes both a data pointer and extra metadata about the data it points to.

Components of a Fat Pointer

- **Data Pointer:**

  - Points to the actual data in memory.
  - Represents the memory address where the data is stored.
- **Extra Metadata:**

  - Additional information such as the length or size of the pointed data.
  - Used for dynamically-sized types (DSTs) like slices (`[T]`) or trait objects (`dyn Trait`), which don't have a fixed size at compile time.

Usage of Fat Pointers:

- **Dynamically Sized Types (DSTs):**
  - Fat pointers are commonly used with types whose size isn't known at compile time, like slices or trait objects.
  - They carry information about the length or traits associated with the type.

```rs
fn main() {
    let data: &[i32] = &[1, 2, 3, 4, 5]; // Slice of integers

    let fat_pointer: *const [i32] = data as *const [i32]; // Creating a fat pointer

    // Not possible: let metadata = fat_pointer.len(); // Fat pointers don't have direct access to metadata

    // Using associated methods to access metadata
    let len = data.len(); // Accessing the length of the slice

    println!("Length: {}", len);
}
```

In this example, `data` is a slice of integers (`&[i32]`). When creating `fat_pointer`, it holds both the memory address of the slice's data and additional metadata about the length of the slice. However, direct access to the metadata from the fat pointer itself isn't possible in Rust; operations involving the metadata usually require the original type or access through the Rust standard library methods.
