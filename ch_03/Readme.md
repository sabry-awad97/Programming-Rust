# Fundamental Types

Despite requiring explicit type annotations for function arguments, return values, struct fields, and certain constructs, Rust provides two features that alleviate the upfront planning required for type specifications:

## Type inference

Type inference allows the Rust compiler to automatically deduce the types of variables and expressions based on the context in which they appear. This means that you don't always have to explicitly specify the types of your variables and expressions, as the compiler can infer them for you. This can make writing Rust code more concise and reduce the amount of boilerplate you have to write.

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let sum = add(5, 6); // sum is inferred to be an i32
```

## Expressive Type System

Rust's type system is expressive and concise, enabling the use of generic types and type aliases to abstract over specific types. This flexibility allows writing generic functions and structures without tightly coupling them to specific types.

```rs
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Self { item }
    }
}
```

## Rust Types

Here's a list of some types in rust

| Type                                | Description                                                                                     | Values                                                                       |
| ----------------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `i8`, `i16`, `i32`, `i64`, `i128`   | Signed integers with 8, 16, 32, 64, and 128 bits of precision, respectively                     | 42, -5i8, 0x400u16, 0o100i16, 20_922_789_888_000u64, b'\*' (u8 byte literal) |
| `u8`, `u16`, `u32`, `u64`, `u128`   | Unsigned integers with 8, 16, 32, 64, and 128 bits of precision, respectively                   | 42, 0x400u16, 20_922_789_888_000u64, b'\*' (u8 byte literal)                 |
| `isize` and `usize`                 | Signed and unsigned integers with the same size as an address on the machine (32 or 64 bits)    | 137, -0b0101_0010isize, 0xffff_fc00usize                                     |
| `f32` and `f64`                     | Single and double precision floating-point numbers, respectively, using the IEEE 754 standard   | 1.61803, 3.14f32, 6.0221e23f64                                               |
| `bool`                              | Boolean                                                                                         | true, false                                                                  |
| `char`                              | Unicode character, 32 bits wide                                                                 | '\*', '\\n', '字', '\\x7f', '\\u{CA0}'                                       |
| `(char, u8, i32)`                   | Tuple: mixed types allowed                                                                      | ('%', 0x7f, -1)                                                              |
| `()`                                | "Unit" (empty tuple)                                                                            | ()                                                                           |
| `struct S { x: f32, y: f32 }`       | Named-field struct                                                                              | S { x: 120.0, y: 209.0 }                                                     |
| `struct T (i32, char)`              | Tuple-like struct                                                                               | T(120, 'X')                                                                  |
| `struct E`                          | Unit-like struct; has no fields                                                                 | E                                                                            |
| `enum Attend { OnTime, Late(u32) }` | Enumeration, algebraic data type                                                                | Attend::Late(5), Attend::OnTime                                              |
| `Box<Attend>`                       | Box: owning pointer to value in heap                                                            | Box::new(Late(15))                                                           |
| `&i32`, `&mut i32`                  | Shared and mutable references: non-owning pointers that must not outlive their referent         | &s.y, &mut v                                                                 |
| `String`                            | UTF-8 string, dynamically sized                                                                 | "ラーメン: ramen".to_string()                                                |
| `&str`                              | Reference to str: non-owning pointer to UTF-8 text                                              | "そば: soba", &s\[0..12\]                                                    |
| `[f64; 4]`, `[u8; 256]`             | Array, fixed length; elements all of same type                                                  | \[1.0, 0.0, 0.0, 1.0\], \[b' '; 256\]                                        |
| `Vec<f64>`                          | Vector, varying length; elements all of same type                                               | vec!\[0.367, 2.718, 7.389\]                                                  |
| `&[u8]`, `&mut [u8]`                | Reference to slice: reference to a portion of an array or vector, comprising pointer and length | &v\[10..20\], &mut a\[..\]                                                   |
| `Option<&str>`                      | Optional value: either `None` (absent) or `Some(v)` (present, with value `v`)                   | `Some("Dr.")`, `None`                                                        |
| `Result<u64, Error>`                | Result of operation that may fail: either a success value `Ok(v)`, or an error `Err(e)`         | `Ok(4096)`, `Err(Error::last_os_error())`                                    |
| `&dyn Any`, `&mut dyn Read`         | Trait object: reference to any value that implements a given set of methods                     | value as `&dyn Any`, `&mut file as &mut dyn Read`                            |
| `fn(&str) -> bool`                  | Pointer to function                                                                             | `str::is_empty`                                                              |

this table lists many of the different types in the Rust programming language

- The `i8`, `i16`, `i32`, `i64`, and `i128` types are signed integers of 8, 16, 32, 64, and 128 bits in size, respectively.
- The `u8`, `u16`, `u32`, `u64`, and `u128` types are unsigned integers of 8, 16, 32, 64, and 128 bits in size, respectively.
- The `isize` and `usize` types are signed and unsigned integers, respectively, that are the same size as an address on the machine. On a 32-bit machine, these types are 32 bits wide, while on a 64-bit machine, they are 64 bits wide.
- The `f32` and `f64` types are IEEE floating-point numbers, representing single and double precision, respectively.
- The `bool` type represents a boolean value, which can be either `true` or `false`.
- The `char` type represents a Unicode character, which is 32 bits wide.
- The `()` type, also known as the unit type, represents an empty tuple. It is often used to represent a value that does not have any meaningful data associated with it.

In addition to these primitive types, Rust also has a number of compound types, including:

- Structs, which are user-defined types that can have multiple fields of different types.
- Tuples, which are fixed-length collections of values of different types.
- Enums, which are types that can have a fixed set of values, each of which can have an optional payload of data.
- Boxes, which are smart pointers that own a value in the heap.
- References, which are non-owning pointers to a value.
- Arrays, which are fixed-length collections of values of the same type.
- Vectors, which are dynamically-sized arrays.
- Slices, which are references to a portion of an array or vector.
- Option&lt;T&gt;, which is a type that can either be `Some(T)` or `None`, representing a value that may or may not be present.
- Result&lt;T, E&gt;, which is a type that represents the result of an operation that may fail, with a success value of type `T` or an error value of type `E`.
- Trait objects, which are references to any value that implements a given set of methods.
- Functions, which are pointers to a block of code that can be called.

## Types categories

In Rust, there are several types, which can be broadly classified into the following categories:

1. Scalar types: These are the most basic types in Rust and represent a single value. They include integers (e.g. `i32`, `u64`), floating-point numbers (e.g. `f32`, `f64`), Booleans (`bool`), and characters (`char`).

1. Compound types: These types are composed of multiple values and can be either owned or borrowed. They include arrays (e.g. `[i32; 5]`), tuples (e.g. `(i32, f32)`), and structs (e.g. `struct Point { x: i32, y: i32 }`).

1. Reference types: These types represent a reference to a value, either owned or borrowed. They include `&T`, `&mut T`, and `Box<T>`.

1. Pointer types: These types represent a pointer to a value, either owned or borrowed. They include `*const T` and `*mut T`.

1. Closure types: These types represent anonymous functions that can be stored, passed around, and called like regular values. They include `Fn`, `FnMut`, and `FnOnce`.

1. Trait types: These types represent a set of behaviors that a type can implement. They are used to define traits, which are like interfaces in other languages.

1. Unsafe types: These types are used to represent low-level concepts that are not safe to use in Rust. They include `*const T` and `*mut T`, as well as the `unsafe` keyword.

Here is a summary of the categories

| Type      | Description                         | Example                                                     |
| --------- | ----------------------------------- | ----------------------------------------------------------- |
| Scalar    | Single values                       | `i32`, `f64`, `bool`, `char`                                |
| Compound  | Multiple values                     | `[i32; 5]`, `(f32, i64)`, `struct Point { x: i32, y: i32 }` |
| Reference | References to values                | `&T`, `&mut T`, `Box<T>`                                    |
| Pointer   | Pointers to values                  | `*const T`, `*mut T`                                        |
| Closure   | Anonymous functions                 | `Fn`, `FnMut`, `FnOnce`                                     |
| Trait     | Behaviors that a type can implement | `Copy`, `Debug`, `IntoIterator`                             |
| Unsafe    | Low-level concepts                  | `*const T`, `*mut T`, `unsafe`                              |

## Fixed-Width Numeric Types

### Integer Types

In Rust, the `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, and `i128` types are fixed-width numeric types. These types represent integers with a fixed number of bits, which means that they can represent a range of values that is determined by the number of bits they use.

For example, an `i8` is a signed 8-bit integer, which means it can represent any integer value between -128 and 127, inclusive. An `u16` is an unsigned 16-bit integer, which means it can represent any integer value between 0 and 65535, inclusive.

Fixed-width numeric types are useful when you need to ensure that a value will always be stored in a specific number of bits. This can be important in situations where the size of the value is critical, such as when working with hardware devices or when communicating with other systems.

It's also worth noting that Rust has type aliases for the fixed-width integer types that correspond to the native word size of the target architecture. These types are `isize` and `usize`, and they are either 32 or 64 bits wide depending on the architecture. These types are typically used when you need an integer type that is the same size as a pointer on the machine.

Here is a summary of the integer types in Rust, along with some example uses of each type:

| Type     | Width (bits) | Range                                                                               | Example Uses                                          |
| -------- | ------------ | ----------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `i8`     | 8            | -128 to 127                                                                         | Small integers, counts, flags, enumerations           |
| **`u8`** | 8            | 0 to 255                                                                            | Small counts, flags, enumerations                     |
| `i16`    | 16           | -32768 to 32767                                                                     | Medium-sized integers, counts, flags, enumerations    |
| `u16`    | 16           | 0 to 65535                                                                          | Medium-sized counts, flags, enumerations              |
| `i32`    | 32           | -2147483648 to 2147483647                                                           | Large integers, counts, flags, enumerations           |
| `u32`    | 32           | 0 to 4294967295                                                                     | Large counts, flags, enumerations                     |
| `i64`    | 64           | -9223372036854775808 to 9223372036854775807                                         | Very large integers, counts, flags, enumerations      |
| `u64`    | 64           | 0 to 18446744073709551615                                                           | Very large counts, flags, enumerations                |
| `i128`   | 128          | -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727 | Extremely large integers, counts, flags, enumerations |
| `u128`   | 128          | 0 to 340282366920938463463374607431768211455                                        | Extremely large counts, flags, enumerations           |
| `isize`  | 32 or 64     | depends on architecture                                                             | Pointer-sized integers                                |
| `usize`  | 32 or 64     | depends on architecture                                                             | Pointer-sized counts                                  |

Here are some examples of using the fixed-width numeric types in Rust:

```rust
// Declare and initialize variables with fixed-width integer types
let x: i8 = 42;
let y: u8 = 255;
let z: i16 = -32768;

// Perform arithmetic operations with fixed-width integer types
let a: i32 = x + y as i32; // Convert y to i32 before adding
let b: u16 = z as u16 + 1; // Convert z to u16 before adding

// Compare fixed-width integer types
let c: bool = x < y as i8; // Convert y to i8 before comparing
let d: bool = z == -32768;

// Convert between fixed-width integer types
let e: u8 = x as u8; // Convert x to u8
let f: i16 = y as i16; // Convert y to i16

// Use type aliases for the native word size
let g: isize = -9223372036854775808;
let h: usize = 18446744073709551615;
```

### Byte literals

- In Rust, a byte literal is a way to represent a single byte value in a string or character literal.

- Byte literals can only represent ASCII and Unicode characters that can be encoded as a single byte in UTF-8. This means that they can't be used to represent characters that require multiple bytes to encode, such as most non-ASCII Unicode characters.

- Byte literals are used to represent the raw byte value of a character in a string or character literal. This can be useful when working with non-UTF-8 encodings, or when you need to manipulate individual bytes in a string.

- Byte literals are written as a single ASCII character preceded by a `b` and enclosed in single quotes. For example, the byte literal for the ASCII character A is b'A'.

  ```rust
  let b1: u8 = b'A'; // A byte literal representing the ASCII character 'A'
  let b2: u8 = b'\n'; // A byte literal representing the ASCII character '\n'
  let b3: u8 = b'\x7f'; // A byte literal representing the ASCII character '\x7f'
  let b4: u8 = b'\xff'; // A byte literal representing the ASCII character '\xff'

  let s1: &[u8] = b"hello"; // A byte string literal representing the ASCII string "hello"
  let s2: &[u8] = b"\x00\x01\x02\x03"; // A byte string literal representing the bytes [0x00, 0x01, 0x02, 0x03]
  ```

- Byte literals can also be written using escape sequences, which allow you to represent certain ASCII and Unicode characters using special codes. Some common escape sequences that can be used in byte literals are:

  | Character         | Byte Literal  | Numeric Equivalent   |
  | ----------------- | ------------- | -------------------- |
  | Single quote, '   | b'''          | 39u8                 |
  | Backslash, \      | b'\\'         | 92u8                 |
  | Newline           | b'\\n'        | 10u8                 |
  | Carriage return   | b'\\r'        | 13u8                 |
  | Tab               | b'\\t'        | 9u8                  |
  | ASCII character   | b'\\xhh'      | hh (hexadecimal)     |
  | Unicode character | b'\\u{hhhhh}' | hhhhhh (hexadecimal) |

  Note that these byte literals are not the same as ASCII character literals, which are written using regular characters and are not preceded by a `b`. For example, the ASCII character literal for the single quote character is `'\''` (with no `b`), and the numeric equivalent is 39i8.

In Rust, you can use the `as` operator to convert from one integer type to another. This can be useful when you need to perform arithmetic or other operations on integers with different precisions or when you need to match the type of an integer to a particular function or API.

```rust
let x: i32 = 42;
let y: i64 = x as i64;

println!("x = {} (i32), y = {} (i64)", x, y);
```

This will print `x = 42 (i32), y = 42 (i64)`.

It's also possible to use the `as` operator to convert from an unsigned integer type to a signed integer type, as long as the value is within the range of the signed type. For example:

```rust
let x: u32 = 42;
let y: i32 = x as i32;

println!("x = {} (u32), y = {} (i32)", x, y);
```

This will also print `x = 42 (u32), y = 42 (i32)`.
