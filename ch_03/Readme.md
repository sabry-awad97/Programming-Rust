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

### Checked, Wrapping, Saturating, and Overflowing Arithmetic

In computer arithmetic, an "overflow" condition occurs when the result of an arithmetic operation exceeds the maximum value that can be represented by the type being used. For example, in Rust, the maximum value that can be represented by an `u8` type is `255`. If you try to perform an arithmetic operation that would produce a result greater than 255, an overflow condition occurs.

An "underflow" condition is the opposite of an overflow condition: it occurs when the result of an arithmetic operation is less than the minimum value that can be represented by the type being used. For example, in Rust, the minimum value that can be represented by an i8 type is -128. If you try to perform an arithmetic operation that would produce a result less than -128, an underflow condition occurs.

In Rust, you can perform arithmetic operations on integers using the standard arithmetic operators (e.g., `+`, `-`, `*`, `/`, `%`). By default, these operators will perform "unchecked" arithmetic, which means that they will not check for overflow or underflow conditions. This can be useful when you want to maximize performance, but it can also be dangerous if you are not careful, as it can lead to undefined behavior if an overflow or underflow occurs.

```rust
// Overflow example
let x: u8 = 255;
let y: u8 = 1;

let result = x + y;

// The result of this operation is 256, which is greater than the maximum value that can be represented by an u8 (255).
// Therefore, an overflow condition occurs, and the result of the operation is undefined.

// Underflow example
let a: u8 = 0; // Minimum value for u8
let b: u8 = 1;

let result = a - b;

// The result of this operation is -1, which is lower than the minimum value that can be represented by an u8 (0).
// Therefore, an underflow condition occurs, and the result of the operation is undefined.
```

Arithmetic operations on numeric types can handle overflow in various ways, offering different behaviors to suit specific needs. Here are the methods Rust provides to handle arithmetic overflow:

1. **Checked Arithmetic (checked\_\* methods)**:

   - Rust's checked arithmetic methods return an Option type. They perform operations and return Some(result) if the operation succeeds within the numeric bounds. If an overflow occurs, they return None.

   ```rs
   let result = a.checked_add(b); // Returns Some(result) if addition doesn't overflow
   ```

2. **Wrapping Arithmetic (wrapping\_\* methods)**:

   - Wrapping arithmetic in Rust doesn't panic on overflow. Instead, it performs the operation and wraps around the value within the size constraints of the data type.
   - For example, if the result exceeds the maximum value for an integer type, it wraps around to the minimum value (or vice versa).

   ```rs
   let result = a.wrapping_add(b); // Performs addition with wrapping behavior on overflow
   ```

3. **Saturating Arithmetic (saturating\_\* methods)**:

   - Saturating arithmetic prevents overflow by saturating the result at the maximum or minimum value of the type.
   - When an operation exceeds the numeric bounds, it clamps the result to the maximum or minimum representable value instead of wrapping or panicking.

   ```rs
   let result = a.saturating_add(b); // Performs addition with saturation, clamping the result
   ```

4. **Overflowing Arithmetic (overflowing\_\* methods)**:

   - Rust's overflowing arithmetic returns a tuple containing the result and a boolean indicating whether an overflow occurred.
   - It performs the arithmetic operation and flags whether the result exceeds the type's bounds.

   ```rs
   let (result, overflowed) = a.overflowing_add(b); // Returns a tuple with the result and an overflow flag
   ```

Here is a summary of the operation names that follow the `checked_`, `wrapping_`, `saturating_`, or `overflowing_` prefix:

| Operation      | Prefix            | Description                                                                                                          |
| -------------- | ----------------- | -------------------------------------------------------------------------------------------------------------------- |
| Addition       | `checked_add`     | Performs checked addition, returning `None` if an overflow or underflow occurs.                                      |
|                | `wrapping_add`    | Performs wrapping addition, wrapping around on overflow or underflow.                                                |
|                | `saturating_add`  | Performs saturating addition, returning the maximum or minimum value if an overflow or underflow occurs.             |
|                | `overflowing_add` | Performs overflowing addition, returning a boolean value indicating whether an overflow or underflow occurred.       |
| Subtraction    | `checked_sub`     | Performs checked subtraction, returning `None` if an overflow or underflow occurs.                                   |
|                | `wrapping_sub`    | Performs wrapping subtraction, wrapping around on overflow or underflow.                                             |
|                | `saturating_sub`  | Performs saturating subtraction, returning the maximum or minimum value if an overflow or underflow occurs.          |
|                | `overflowing_sub` | Performs overflowing subtraction, returning a boolean value indicating whether an overflow or underflow occurred.    |
| Multiplication | `checked_mul`     | Performs checked multiplication, returning `None` if an overflow or underflow occurs.                                |
|                | `wrapping_mul`    | Performs wrapping multiplication, wrapping around on overflow or underflow.                                          |
|                | `saturating_mul`  | Performs saturating multiplication, returning the maximum or minimum value if an overflow or underflow occurs.       |
|                | `overflowing_mul` | Performs overflowing multiplication, returning a boolean value indicating whether an overflow or underflow occurred. |
| Division       | `checked_div`     | Performs checked division, returning `None` if an overflow or underflow occurs or if the divisor is zero.            |
|                | `wrapping_div`    | Performs wrapping division, wrapping around on overflow or underflow or if the divisor is zero.                      |
| Remainder      | `checked_rem`     | Performs checked remainder, returning `None` if an overflow or underflow occurs or if the divisor is zero.           |
|                | `wrapping_rem`    | Performs wrapping remainder, wrapping around on overflow or underflow or if the divisor is zero.                     |
| Left shift     | `checked_shl`     | Performs checked left shift, returning `None` if an overflow or underflow occurs.                                    |
|                | `wrapping_shl`    | Performs wrapping left shift, wrapping around on overflow or underflow.                                              |
|                | `overflowing_shl` | Performs overflowing left shift, returning a boolean value indicating whether an overflow or underflow occurred.     |
| Right shift    | `checked_shr`     | Performs checked right shift, returning `None` if an overflow or underflow occurs.                                   |
|                | `wrapping_shr`    | Performs wrapping right shift, wrapping around on overflow or underflow.                                             |
|                | `overflowing_shr` | Performs overflowing right shift, returning a boolean value indicating whether an overflow or underflow occurred.    |

```rs
fn main() {
    // Addition
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_add(y);
    let wrapping_result = x.wrapping_add(y);
    let saturating_result = x.saturating_add(y);
    let (overflowing_result, overflowed) = x.overflowing_add(y);

    // Subtraction
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_sub(y);
    let wrapping_result = x.wrapping_sub(y);
    let saturating_result = x.saturating_sub(y);
    let (overflowing_result, overflowed) = x.overflowing_sub(y);

    // Multiplication
    let x = 2u8;
    let y = 3u8;

    let checked_result = x.checked_mul(y);
    let wrapping_result = x.wrapping_mul(y);
    let saturating_result = x.saturating_mul(y);
    let (overflowing_result, overflowed) = x.overflowing_mul(y);

    assert_eq!(checked_result, Some(6));
    assert_eq!(wrapping_result, 6);
    assert_eq!(saturating_result, 6);
    assert_eq!(overflowing_result, 6);
    assert!(!overflowed);

    // Division
    let x = 10u8;
    let y = 3u8;

    let checked_result = x.checked_div(y);
    let wrapping_result = x.wrapping_div(y);

    // Remainder
    let x = 10u8;
    let y = 3u8;

    let checked_result = x.checked_rem(y);
    let wrapping_result = x.wrapping_rem(y);

    // Left shift
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_shl(y as u32);
    let wrapping_result = x.wrapping_shl(y as u32);
    let (overflowing_result, overflowed) = x.overflowing_shl(y as u32);

    // Right shift
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_shr(y as u32);
    let wrapping_result = x.wrapping_shr(y as u32);
    let (overflowing_result, overflowed) = x.overflowing_shr(y as u32);
}
```

## Floating Point Types

In Rust, there are two main types of floating-point numbers: `f32` for single-precision floating point and `f64` for double-precision floating point. Both types are based on the IEEE 754 standard and have the following properties:

- They can represent positive and negative infinity, as well as “not a number” (NaN) values.
- Floating-point numbers are represented using a sign bit, an exponent, and a fractional part (mantissa).
- They provide a relatively high precision for representing real numbers, but they do have limitations due to the fixed number of bits they use.
- Both `f32` and `f64` implement the `Float` trait, which provides a number of methods for working with floating-point numbers, such as `abs`, `ceil`, `floor`, and others.

Here are some examples of using floating-point types in Rust:

```rust
fn main() {
    let x: f64 = 2.0;
    let y: f32 = 3.0;

    // Error! No implicit conversion
    // let z = x + y;

    // addition
    let sum = x + y as f64;

    // subtraction
    let difference = x - y as f64;

    // multiplication
    let product = x * y as f64;

    // division
    let quotient = x / y as f64;

    // remainder
    let remainder = x % y as f64;
}
```

In the above example, the variable `x` is of type `f64`, while `y` is of type `f32`. When we try to add `x` and `y` directly, we get a compile-time error because there is no implicit conversion between these two types. We can fix the error by explicitly casting `x` to an `f32` using the `as` operator.

Keep in mind that floating-point numbers can be imprecise due to the fixed number of bits they use to represent their mantissa and exponent. You should be aware of this limitation when using them in your code.

| Type  | Description                                                      | Mantissa Bits | Exponent Bits | Precision               |
| ----- | ---------------------------------------------------------------- | ------------- | ------------- | ----------------------- |
| `f32` | Single-precision floating point, based on the IEEE 754 standard. | 24            | 8             | Approximately 7 digits  |
| `f64` | Double-precision floating point, based on the IEEE 754 standard. | 53            | 11            | Approximately 15 digits |

- The `f32` and `f64` types both have associated constants that represent special floating-point values, such as positive and negative infinity, the not-a-number (NaN) value, and the minimum and maximum finite values.

  Here are some examples of using these constants:

  ```rust
  use std::f32;

  let x = f32::INFINITY; // x is positive infinity
  let y = f32::NEG_INFINITY; // y is negative infinity
  let z = f32::NAN; // z is the NaN value
  let w = f32::MIN; // w is the smallest finite f32 value
  let v = f32::MAX; // v is the largest finite f32 value
  ```

- You can also check whether a floating-point value is one of these special values using the `is_infinite`, `is_nan`, and `is_finite` methods provided by the `Float` trait. For example:

  ```rust
  fn main() {
      let x = f32::INFINITY;
      assert!(x.is_infinite());

      let y = f32::NAN;
      assert!(y.is_nan());

      let z: f32 = 1.0;
      assert!(z.is_finite());
  }
  ```

- The f32 and f64 types provide a full complement of methods for mathematical calculations. The `std::f32::consts` and `std::f64::consts` modules in the Rust standard library provide various commonly used mathematical constants such as `E`, `PI`, and `SQRT_2` as constants that you can use in your Rust code. These constants are provided for both the `f32` and `f64` types.

  ```rust
  use std::f64::consts::{E, PI, SQRT_2};

  fn main() {
      let x = 2f64;

      // Calculate the square root of x
      let x_sqrt = x.sqrt();

      // Calculate the sine of x
      let x_sin = x.sin();

      // Calculate the cosine of x
      let x_cos = x.cos();

      // Calculate the tangent of x
      let x_tan = x.tan();

      // Calculate the natural logarithm of x
      let x_ln = x.ln();

      // Calculate the base-10 logarithm of x
      let x_log10 = x.log10();

      // Calculate the base-2 logarithm of x
      let x_log2 = x.log2();

      // Calculate the power of E raised to the x
      let e_to_the_x = E.powf(x);

      // Calculate the absolute value of x
      let x_abs = x.abs();

      // Calculate the maximum of x and PI
      let max = x.max(PI);

      // Calculate the minimum of x and SQRT_2
      let min = x.min(SQRT_2);
  }
  ```

  In this example, we use the `sqrt`, `sin`, `cos`, `tan`, `ln`, `log10`, `log2`, `powf`, `abs`, `max`, and `min` methods on the `f64` type, as well as the `E`, `PI`, and `SQRT_2` constants from the `std::f64::consts` module.

- In Rust, the precedence of method calls is higher than the precedence of prefix operators such as `!` or `-`. This means that if you have a method call on a negated value, you need to use parentheses to ensure that the method call is evaluated before the negation.

  For example, consider the following code:

  ```rust
  let x = 2_f32;
  let y = -x.abs();

  println!("{}", y);
  ```

  In this code, the `abs` method is called on `-x`, which returns the absolute value of `x`. However, because the precedence of the method call is higher than the precedence of the negation operator, the `abs` method is called first and then the result is negated. This means that the value of `y` will be `-2`, not `2`.

  To correctly evaluate this expression, you need to use parentheses to specify that the negation should be applied first:

  ```rust
  let x = 2_f32;
  let y = (-x).abs();

  println!("{}", y);
  ```

  Now, the value of `y` will be `2`, as expected.

  It's always a good idea to use parentheses to clarify the order of operations in your code, especially when using multiple operators or method calls. This can help prevent confusion and reduce the chance of errors.

## The bool Type

The `bool` type is used to represent a binary choice or a logical value in Rust.

### Characteristics

1. **Two Possible Values**:

   - `true`: Represents the logical truth.
   - `false`: Represents the logical falsehood.

1. **Memory Representation**:

   - Typically occupies 1 byte in memory.
   - The `bool` type is stored as `1` for `true` and `0` for `false`.

1. **Logical Operations**:

   - Supports logical operations like `&&` (AND), `||` (OR), and `!` (NOT) for Boolean logic.

**Example Usage**:

```rs
fn main() {
    let is_rust_fun: bool = true; // Declaring a bool variable

    if is_rust_fun {
        println!("Rust is fun!"); // Executes if the value is true
    } else {
        println!("Rust is not fun!"); // Executes if the value is false
    }
}
```

**Negating the value of a `bool` variable**.

```rs
fn main() {
    let x = true;
    let y = false;

    // Negate the value of x
    let x_not = !x; // x_not is false

    // Negate the value of y
    let y_not = !y; // y_not is true
}
```

In this example, we use the `!` operator to negate the value of `x` and `y`. The negation of `true` is `false`, and the negation of `false` is `true`.

**Checking for equality and inequality between `bool` variables**:

```rust
fn main() {
    let x = true;
    let y = false;

    // Check if x and y are equal
    let x_eq_y = x == y; // x_eq_y is false

    // Check if x and y are not equal
    let x_ne_y = x != y; // x_ne_y is true
}
```

In this example, we use the `==` and `!=` operators to check for equality and inequality between `x` and `y`. The `==` operator returns `true` if the two operands are equal, and `false` otherwise. The `!=` operator returns `true` if the two operands are not equal, and `false` otherwise.

**Performing logical AND and OR operations on `bool` variables**:

```rust
fn main() {
    let x = true;
    let y = false;

    // Check if x is true and y is false
    let x_and_y = x && y; // x_and_y is false

    // Check if x is true or y is false
    let x_or_y = x || y; // x_or_y is true
}
```

In this example, we use the `&&` and `||` operators to perform logical AND and OR operations on `x` and `y`. The `&&` operator returns `true` if both operands are `true`, and `false` otherwise. The `||` operator returns `true` if either operand is `true`, and `false` otherwise.

**Using the `bool` type in conditional statements**:

```rust
fn main() {
    let x = true;
    let y = false;

    if x {
        println!("x is true");
    } else {
        println!("x is false");
    }

    if y {
        println!("y is true");
    } else {
        println!("y is false");
    }
}
```

In this example, we use the `if` statement to control the flow of the program based on the value of `x` and `y`. The `if` statement will execute the block of code following the `if` keyword if the condition is `true`, and the block of code following the `else` keyword if the condition is `false`. In this case, the first `if` statement will execute the block following the `if` keyword because `x` is `true`, and the second `if` statement will execute the block following the `else` keyword because `y` is `false`.

**Using the `bool` type in a `match` statement**:

```rust
fn main() {
    let x = true;

    match x {
        true => println!("x is true"),
        false => println!("x is false"),
    }
}
```

In this example, we use a `match` statement to match the value of `x` against a set of patterns. The `match` statement will execute the code block associated with the first pattern that matches the value of `x`. In this case, the code block following the `true` pattern will be executed because `x` is `true`.

**Using the `as` operator to convert a bool value to an integer type**:

```rust
fn main() {
    let x = true;

    // Convert x to an i32
    let x_i32 = x as i32; // x_i32 is 1

    // Convert x to a u8
    let x_u8 = x as u8; // x_u8 is 1
}
```

Here is a summary of the main features of the bool type in Rust:
| Feature | Description |
| --- | --- |
| Values | The `bool` type has two possible values: `true` and `false`. |
| Operators | The `bool` type supports the following operators: `!` (negation), `==` (equality), `!=` (inequality), `&&` (logical AND), `|
| Conversion | The`bool`type can be converted to an integer type using the`as`operator.`true`is converted to`1`and`false`is converted to`0`. |
| Use in conditional statements | The `bool`type can be used in conditional statements such as`if`and`match`to control the flow of the program based on a boolean value. |
| Use in functions | The`bool`type can be used as the return type for functions that return a boolean value. |
| Printing | The`bool`type can be printed using the`{}`format specifier in the`println!`macro.`println!("{}", true)`|
| Parsing | The`bool`type can be parsed from a string using the`parse`method.`"true".parse::<bool>().unwrap()` |

## Characters

The `char` type represents a Unicode scalar value, which is a unique integer value that represents a character. Some key points about the `char` type in Rust are:

1. `char` values are 4 bytes in size, which means they can represent any Unicode scalar value.

   ```rs
   fn main() {
       let c: char = 'a';
       println!("Size of 'a': {} bytes", std::mem::size_of_val(&c));
   }
   ```

1. `char` values can be created using single quotes, like `'a'` or `'😄'`.

   ```rs
   fn main() {
       let c: char = 'a';
       let d: char = '😄';
       println!("{}", c);
       println!("{}", d);
   }
   ```

1. `char` values are stored as Unicode Scalar Value (USV) in Rust, which is a unique integer value that represents a character in the Unicode standard.

   ```rs
   fn main() {
       let c: char = 'a';
       println!("USV of 'a': {}", c as u32);
   }
   ```

1. The `char` type implements the `FromStr` trait, which means that you can parse a `char` value from a string using the `str::parse` method.

   ```rs
   fn main() {
       let c: char = "a".parse().unwrap();
       let d: char = "😄".parse().unwrap();
   }
   ```

1. You can convert a `char` value to and from its corresponding integer value using the `ascii` and `from_u32` methods, respectively.

   ```rs
   fn main() {
       let c: char = 'a';
       let i: u32 = c as u32;
       let d: char = char::from_u32(i).unwrap();
       assert_eq!(c, d);
   }
   ```

1. The `char` type implements the `PartialEq`, `Eq`, and `Ord` traits, which means that you can compare `char` values using the comparison operators (e.g., `==`, `!=`, `<`, `>`, etc.) and the `cmp` method.

   ```rs
   fn main() {
       let a: char = 'a';
       let b: char = 'b';
       assert!(a < b);
       assert!(a != b);
   }
   ```

1. You can iterate over the characters in a string using the `chars` method of the `str` type, which returns an iterator of `char` values.

   ```rs
   fn main() {
       for c in "hello".chars() {
           println!("{}", c);
       }
   }
   ```

1. The `char` type has an associated constant called `MAX`, which represents the largest possible `char` value:

   ```rs
   fn main() {
       let c: char = char::MAX;
       println!("Largest possible char: {}", c);
   }
   ```

1. The `char` type has a number of methods for checking its properties, such as `is_alphabetic`, `is_alphanumeric`, `is_numeric`, `is_uppercase`, `is_lowercase`, `is_whitespace`, and `is_control`. These methods can be useful for validating input or for filtering or sorting strings.

   ```rs
   fn main() {
       let c: char = 'a';
       assert!(c.is_alphabetic());

       let d: char = '1';
       assert!(d.is_numeric());

       let e: char = ' ';
       assert!(e.is_whitespace());

       let f: char = '\u{0000}';
       assert!(f.is_control());
   }
   ```

1. The `char` type has a number of methods for manipulating its case:

```rs
fn main() {
    let c: char = 'a';
    let d: char = c.to_uppercase().next().unwrap();
    assert_eq!(d, 'A');

    let e: char = 'A';
    let f: char = e.to_lowercase().next().unwrap();
    assert_eq!(f, 'a');

    let g: char = 'h';
    let h: char = g.to_ascii_uppercase();
    assert_eq!(h, 'H');
}
```

1. The `char` type has a number of methods for converting its value to and from various representations:

   ```rs
   fn main() {
       let c: char = 'a';
       let s1: String = c.escape_unicode().collect();
       assert_eq!(s1, "\\u{61}");

       let s2: String = c.escape_debug().collect();
       assert_eq!(s2, "a");

       let s3: String = c.escape_default().collect();
       assert_eq!(s3, "a");

       let d: char = '\u{1f600}';
       let s4: String = d.escape_unicode().collect();
       assert_eq!(s4, "\\u{1f600}");

       let s5: String = d.escape_debug().collect();
       assert_eq!(s5, "😀");

       let s6: String = d.escape_default().collect();
       assert_eq!(s6, "\\u{1f600}");

       let s7: String = "a".to_string();
       let f: char = s7.parse().unwrap();
       assert_eq!(f, 'a');
   }
   ```

1. The char type implements the Copy and Clone traits

   ```rust
   fn main() {
       let c: char = 'a';
       let d = c;
       assert_eq!(c, d);
   }
   ```

## Tuple Type

A tuple is a compound data type that can contain a fixed number of values of different types. Tuples can be created using parentheses and a comma-separated list of values.

Here is a summary of the main points about the tuple type in Rust:

1. Tuples can be created using the `(a, b, c, ...)` syntax, where `a`, `b`, `c`, etc. are the values in the tuple.

   ```rs
   fn main() {
       let t = (1, "hello", std::f64::consts::PI);
   }
   ```

1. A tuple is an ordered collection of values with potentially different types.

   ```rs
   fn main() {
       let t = (1, "hello", std::f64::consts::PI);
   }
   ```

1. Tuples can be destructured using pattern matching, allowing you to extract the values from a tuple and bind them to variables.

   ```rs
   fn main() {
       let t = (1, "hello", std::f64::consts::PI);
       let (x, y, z) = t;
       assert_eq!(x, 1);
       assert_eq!(y, "hello");
       assert_eq!(z, std::f64::consts::PI);
   }
   ```

1. Tuples can be accessed using indexing, allowing you to retrieve the value at a specific position in the tuple.

   ```rs
   fn main() {
       let t = (1, "hello", std::f64::consts::PI);
       let x = t.0;
       assert_eq!(x, 1);

       let y = t.1;
       assert_eq!(y, "hello");

       let z = t.2;
       assert_eq!(z, std::f64::consts::PI);
   }
   ```

1. Tuples have a fixed size, determined by the number of values they contain.

   ```rs
   fn tuple_length<T: std::fmt::Debug + Sized>(tuple: &T) -> usize {
       let s = format!("{:#?}", tuple);
       println!("{:#?}", s);
       match s.contains(',') {
           true => {
               let parts = s.matches(',').collect::<Vec<&str>>();
               parts.len()
           }
           _ => 1,
       }
   }

   fn main() {
       let t = (1, "hello", std::f64::consts::PI);
       println!("Number of tuple elements: {}", tuple_length(&t));
   }
   ```

1. Tuples can be used as the return type of a function to allow the function to return multiple values.

   ```rs
   fn divmod(x: i32, y: i32) -> (i32, i32) {
       (x / y, x % y)
   }

   fn main() {
       let (q, r) = divmod(10, 3);
       assert_eq!(q, 3);
       assert_eq!(r, 1);
   }
   ```

1. Tuples can be used as the elements of an array or vector, allowing you to create a collection of ordered groups of values.

   ```rs
   fn main() {
       let v = [(1, 2), (3, 4), (5, 6)];
       assert_eq!(v[0], (1, 2));
       assert_eq!(v[1], (3, 4));
       assert_eq!(v[2], (5, 6));
   }
   ```

1. Tuples can be used as the elements of a `HashMap`, allowing you to create a mapping from ordered groups of values to other values:

   ```rs
   fn main() {
       use std::collections::HashMap;

       let mut m = HashMap::new();
       m.insert((1, 2), "foo");
       m.insert((3, 4), "bar");

       assert_eq!(m.get(&(1, 2)), Some(&"foo"));
       assert_eq!(m.get(&(3, 4)), Some(&"bar"));
   }
   ```

1. Tuples can be used as the keys of a `BTreeMap`, allowing you to create an ordered mapping from ordered groups of values to other values:

   ```rs
   use std::collections::BTreeMap;

   fn main() {
       let mut m = BTreeMap::new();
       m.insert((1, 2), "foo");
       m.insert((3, 4), "bar");

       for (k, v) in m.iter() {
           println!("{:?}: {}", k, v);
       }
   }
   ```

1. Tuples can be used as the elements of a `Set`, allowing you to create a collection of unique ordered groups of values:

   ```rs
   use std::collections::HashSet;

   fn main() {
       let mut s = HashSet::new();
       s.insert((1, 2));
       s.insert((3, 4));

       assert!(s.contains(&(1, 2)));
       assert!(s.contains(&(3, 4)));
   }
   ```

1. Tuples can be used as the keys of a `BTreeSet`, allowing you to create an ordered collection of unique ordered groups of values:

   ```rs
   use std::collections::BTreeSet;

   fn main() {
       let mut s = BTreeSet::new();
       s.insert((1, 2));
       s.insert((3, 4));

       for x in s.iter() {
           println!("{:?}", x);
       }
   }
   ```

1. Tuples can be used as the elements of a `Queue`, allowing you to create a FIFO (first-in, first-out) data structure with ordered groups of values:

   ```rs
   use std::collections::VecDeque;

   fn main() {
       let mut q = VecDeque::new();
       q.push_back((1, 2));
       q.push_back((3, 4));

       assert_eq!(q.pop_front(), Some((1, 2)));
       assert_eq!(q.pop_front(), Some((3, 4)));
       assert_eq!(q.pop_front(), None);
   }
   ```

1. Tuples can be used as the elements of a `Stack`, allowing you to create a LIFO (last-in, first-out) data structure with ordered groups of values:

   ```rs
   use std::collections::VecDeque;

   fn main() {
       let mut s = VecDeque::new();
       s.push_back((1, 2));
       s.push_back((3, 4));

       assert_eq!(s.pop_back(), Some((3, 4)));
       assert_eq!(s.pop_back(), Some((1, 2)));
       assert_eq!(s.pop_back(), None);
   }
   ```

1. The `std::mem` module provides a number of functions for inspecting and manipulating tuples, including `size_of_val`, `align_of_val`, and `swap`:

   ```rs
   use std::mem;

   fn main() {
       let mut t = (1, 2, 3);

       let size = mem::size_of_val(&t);
       assert_eq!(size, 3 * mem::size_of::<i32>());

       let align = mem::align_of_val(&t);
       assert_eq!(align, mem::align_of::<i32>());

       let mut u = (4, 5, 6);
       mem::swap(&mut t, &mut u);
       assert_eq!(t, (4, 5, 6));
       assert_eq!(u, (1, 2, 3));
   }
   ```

1. The `std::convert` module provides a number of functions and traits for converting between tuples and other types, including `From`, `Into`, and `TryFrom`:

   ```rs
   use std::convert::*;

   #[derive(Debug, PartialEq)]
   struct Complex {
       real: f64,
       imag: f64,
   }

   impl TryFrom<(f64, f64)> for Complex {
       type Error = &'static str;

       fn try_from(t: (f64, f64)) -> Result<Self, Self::Error> {
           if t.0.is_nan() || t.1.is_nan() {
               Err("invalid value")
           } else {
               Ok(Complex {
                   real: t.0,
                   imag: t.1,
               })
           }
       }
   }

   fn main() {
       let t = (1.0, 2.0);
       let c = Complex::try_from(t);
       assert_eq!(
           c,
           Ok(Complex {
               real: 1.0,
               imag: 2.0
           })
       );

       let t = (f64::NAN, f64::NAN);
       let c = Complex::try_from(t);
       assert_eq!(c, Err("invalid value"));
   }
   ```

1. The `std::cmp` module provides a number of trait implementations for tuples, including `PartialEq`, `Eq`, `PartialOrd`, and `Ord`:

   ```rust
   fn main() {
       let t1 = (1, 2, 3);
       let t2 = (4, 5, 6);

       assert!(t1 != t2);
       assert!(t1 < t2);
       assert!(t1 <= t2);
       assert!(t2 > t1);
       assert!(t2 >= t1);

       let t3 = (1, 2, 3);

       assert!(t1 == t3);
       assert!(t1 <= t3);
       assert!(t1 >= t3);

       let t4 = (1, 2, 4);

       assert!(t1 < t4);
       assert!(t1 <= t4);
       assert!(t4 > t1);
       assert!(t4 >= t1);
   }
   ```

1. The `std::mem` module's `replace` can be used to atomically replace the contents of a tuple with new values. This can be useful for implementing concurrent data structures or for implementing lock-free algorithms.

   ```rust
   use std::mem;

   fn main() {
       let mut t = (1, 2, 3);

       let old = mem::replace(&mut t, (4, 5, 6));
       assert_eq!(old, (1, 2, 3));
       assert_eq!(t, (4, 5, 6));
   }
   ```

1. The `std::cmp` module's `Ord::cmp` function can be used to compare tuples element-by-element. This allows you to specify a custom ordering for tuples that takes into account more than just the first element.

   ```rs
   use std::cmp::*;

   fn main() {
       let t1 = (1, 2, 3);
       let t2 = (4, 5, 6);

       let ord = t1.cmp(&t2);
       assert_eq!(ord, Ordering::Less);

       let t3 = (1, 2, 4);

       let ord = t1.cmp(&t3);
       assert_eq!(ord, Ordering::Equal);
   }
   ```

1. Tuples can contain a single value.

   ```rust
   fn main() {
       let t = (1,);
       let x = t.0;
       assert_eq!(x, 1);
   }
   ```

_**Zero Tuple**_

- The zero-tuple is a special type of tuple that has no elements.

- It is written as `()` and is used to represent the absence of a value.

  ```rust
  let t: () = ();
  ```

- The zero-tuple is often used as the return type of functions that do not need to return a value.

  ```rs
  fn foo() -> () {
      // do something
  }
  ```

- The `std::mem` module's `size_of_val` function returns `0` for the zero-tuple, whereas it returns the size of the tuple's elements for other tuples.

  ```rs
  use std::mem;

  fn main() {
      let size = mem::size_of_val(&());
      assert_eq!(size, 0);

      let size = mem::size_of_val(&(1, 2, 3));
      assert_eq!(size, 3 * mem::size_of::<i32>());
  }
  ```

- The zero-tuple is a special case of the tuple type in Rust and is treated differently in certain contexts.

## Pointer Types

There are three types that represent memory addresses: references, boxes, and unsafe pointers.

- **References (`&T`)** are non-owning pointers that allow you to borrow a value from its owner. References are immutable by default, but you can use `&mut T` to create a mutable reference. References have a limited lifetime, which means that they can only be used as long as the value they refer to is still in scope. Using a reference after its lifetime has ended is undefined behavior and can lead to bugs or crashes.

- **Boxes (`Box<T>`)** are pointers that store a value on the heap. Boxes facilitate transferring ownership across scopes and enabling values to exist beyond their original scope. Boxes allow storing data of sizes known only at runtime, enabling flexible data structures. Boxes enable creating recursive data structures that reference themselves indirectly via `Box<T>`. You can use the `Box::new` function to create a new box.

- **Unsafe pointers (`*const T` or `*mut T`)** are raw pointers that do not have the safety guarantees of references or boxes. Unsafe pointers do not have a lifetime or enforce any rules about how they are used, so you can use them to perform arbitrary operations on memory. They are often used for low-level system programming tasks or when interacting with foreign code. However, using unsafe pointers can lead to undefined behavior if you do not use them correctly, so they should be used with caution.
