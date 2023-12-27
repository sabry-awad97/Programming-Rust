# Error Handling

Rust has a unique approach to error handling compared to other languages, called "Result types". In this approach, the code returns either an "Ok" value, indicating that the function has completed successfully, or an "Err" value, indicating that an error has occurred.

## Panic

The use of `panic!` can make code more difficult to reason about and debug, since it can be difficult to determine where the error occurred and why the program halted. Therefore, it is generally best to reserve the use of `panic!` for situations where it is truly necessary, and to handle recoverable errors using other mechanisms. `panic!` is for the kind of error that should never happen.

A program panics when it encounters an error that is so severe that there must be a bug in the program itself. These errors include things like:

- Out-of-bounds vector access:

  ```rs
  let arr = vec![1, 2, 3];
  let index = 5;
  let value = arr[index];
  // This will panic at runtime with an "index out of bounds" error.
  ```

- Integer division by zero:

  ```rs
  let numerator = 10;
  let denominator = 0;
  let result = numerator / denominator;
  // This will panic at runtime with a "attempt to divide by zero" error.
  ```

- Calling `.expect()` on a Result that happens to be Err:

  ```rs
  use std::fs::File;
  let file = File::open("nonexistent_file.txt").expect("Failed to open file");
  // This will panic at runtime with a custom error message.
  ```

- Assertion failure:

  ```rs
  let x = 5;
  let y = 10;
  assert!(x > y, "x is not greater than y");
  // This will panic at runtime with a custom error message.
  ```

- Triggering a panic directly with `panic!()`:

  ```rs
  let age = -10;
  if age < 0 {
      panic!("Age cannot be negative!");
  }
  // This will panic at runtime with a custom error message.
  ```

When these errors occur, Rust provides two options: _unwinding the stack_ or _aborting the process_. By default, Rust will unwind the stack. This means that Rust will attempt to unwind the stack and clean up any resources that were allocated before the panic occurred. This process involves calling destructors for any objects on the stack and freeing any resources that were allocated on the heap.

However, in some cases, it may be more appropriate to abort the process when a panic occurs. Aborting the process is a more drastic measure, but it can be useful in situations where the program has become so unstable that it is unsafe to continue executing. To abort the process, you can set the panic runtime configuration option to abort using the `panic = "abort"` configuration in your `Cargo.toml` file.

### Unwinding

In Rust, when a panic occurs, the default behavior is for the program to unwind the stack. Unwinding the stack means that Rust will walk back up the stack and call the destructors of any objects that were created along the way, in the reverse order in which they were created. This process ensures that any resources allocated on the heap are properly cleaned up, and any temporary objects are destroyed.

```rs
fn main() {
    let v = vec![1, 2, 3];
    panic!("oh no!");
}
```

When the `panic!` macro is executed, the program will begin unwinding the stack. This means that Rust will call the destructors of any objects that were created along the way, in the reverse order in which they were created. In this case, Rust will destroy the vector `v` before the program exits. This ensures that any memory allocated on the heap for the vector is properly cleaned up before the program terminates.

There is also a way to catch stack unwinding, allowing the thread to survive and continue running. The standard library function `std::panic::catch_unwind()` does this.

This function takes a closure that contains the code you want to run, and returns a `Result` indicating whether the closure panicked or not.

```rs
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("this code is running inside a catch_unwind closure");
        panic!("oh no, something went wrong!");
    });

    match result {
        Ok(()) => println!("the closure ran successfully"),
        Err(_) => println!("the closure panicked"),
    }

    println!("the program continues to run after the catch_unwind closure");
}
```

### Aborting

Unwinding the stack can be expensive in terms of performance, especially if the program has to unwind a large number of objects. For this reason, Rust provides an alternative method for handling panics: aborting the program. Aborting the program simply terminates the process immediately without unwinding the stack. While this can be a faster method for handling panics, it also means that any resources that were allocated during the program's execution will not be properly cleaned up.

You can choose to abort the program instead of unwinding the stack by setting the panic runtime configuration option to abort. This can be done in your Cargo.toml file, like this:

```toml
[profile.release]
panic = "abort"
```

There are three circumstances in which Rust does not try to unwind the stack when a panic occurs:

- When the panic is caused by a call to the `std::process::abort` function. The `std::process::abort` function immediately terminates the process without any attempt to unwind the stack.

- When the program is built with the "panic=abort" configuration option. This configuration option causes the program to immediately terminate the process without any attempt to unwind the stack when a panic occurs.

- If a `drop()` method itself panics, it can trigger a second panic while Rust is still trying to unwind the stack and clean up resources. This is known as a "double panic" and is considered fatal. When a double panic occurs, Rust will immediately stop unwinding and abort the whole process, which can lead to some resources not being properly cleaned up.

To avoid double panics, it's important to ensure that `drop()` methods are implemented in a way that does not panic. One common approach is to use the `std::mem::take()` function to replace the value being dropped with a default value before attempting to drop it. This can prevent panics caused by trying to drop a value that has already been dropped or is in an invalid state.

```rs
#[derive(Default)]
struct MyType {
    // fields and implementations
}

impl MyType {
    fn new() -> Self {
        Self {}
    }
}

impl Drop for MyType {
    fn drop(&mut self) {
        // Code that could potentially panic when dropping MyType
    }
}

fn main() {
    let mut my_value = MyType::new(); // Assume there's a function to create MyType instances

    // Use std::mem::take() to replace my_value with a default value
    let _ = std::mem::take(&mut my_value);
    // Now my_value is reset to a default state, and dropping it won't cause a panic
    // Further operations with my_value might require re-initialization
}
```

## Result Type

Rust doesn’t have exceptions.

In Rust, the `Result` type is a built-in enum that represents either a successful value (`Ok`) or an error (`Err`). Its definition is as follows:

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` is the type of the value returned in case of success, and `E` is the type of the error that occurred.

### Catching Errors with Match Expressions

```rs
let result = do_something_that_returns_a_result();
match result {
    Ok(val) => {
        // Handle the successful case
    }
    Err(err) => {
        // Handle the error case
    }
}
```

### Methods provided by Rust's `Result` type

1. `map`: Applies a function to the `Ok` value of the `Result`, returning a new `Result` with the transformed value. If the `Result` is `Err`, the function is not called and the `Err` is returned unchanged.

   ```rs
   let result: Result<i32, &str> = Ok(42);
   let mapped = result.map(|val| val * 2);
   ```

1. `map_err`: Applies a function to the `Err` value of the `Result`, returning a new `Result` with the transformed error. If the `Result` is `Ok`, the function is not called and the `Ok` value is returned unchanged.

   ```rs
   let result: Result<&str, i32> = Err(42);
   let mapped_err = result.map_err(|err| err * 2);

   fn open_file(path: &str) -> Result<File, std::io::Error> {
       File::open(path).map_err(|err| {
           println!("Failed to open file {}: {}", path, err);
           err
       })
   }

   let file = open_file("nonexistent.txt");
   ```

1. `and`: Unwraps the `Result`, returning the `Ok` value if the `Result` is `Ok`, otherwise returning the `Err` value.

   ```rs
   let result: Result<i32, &str> = Ok(42);
   let and_result = result.and(Ok(100));
   ```

1. `and_then`: Similar to `and`, but applies a function to the `Ok` value and returns the new `Result` produced by that function. If the `Result` is `Err`, the function is not called and the `Err` is returned unchanged.

   ```rs
   use std::{
       fs::File,
       io::{Error, ErrorKind, Read},
   };

   fn main() {
       let result: Result<i32, &str> = Ok(42);
       let and_then_result = result.and_then(|val| Ok(val * 2));

       let read_integer = |file: &mut File| {
           let mut buffer = String::new();
           file.read_to_string(&mut buffer).and_then(|_| {
               buffer.trim().parse::<i32>().map_err(|err| {
                   println!("Failed to parse integer: {}", err);
                   Error::new(ErrorKind::InvalidData, "Invalid integer")
               })
           })
       };

       let mut file = File::open("data.txt").unwrap();
       let integer = read_integer(&mut file);
   }
   ```

1. `or`: Unwraps the `Result`, returning the `Ok` value if the `Result` is `Ok`, otherwise returning the provided default value.

   ```rs
   let result = Err::<i32, &str>("Error message");
   let fallback = Ok(42);

   let value = result.or(fallback);
   println!("Result: {:?}", value);
   ```

1. `or_else`: Similar to `or`, but applies a function to the `Err` value and returns the new `Result` produced by that function. If the `Result` is `Ok`, the function is not called and the `Ok` value is returned unchanged.

   ```rs
   let result: Result<i32, &str> = Err("something went wrong");
   let or_else_result = result.or_else(|err| Ok(100));
   ```

1. `unwrap`: Unwraps the `Ok` value of the `Result`, panicking if the `Result` is `Err`.

   ```rs
   let result: Result<i32, &str> = Ok(42);
   let unwrapped = result.unwrap();
   ```

1. `unwrap_err`: Unwraps the `Err` value of the `Result`, panicking if the `Result` is `Ok`.

   ```rs
   let result: Result<&str, i32> = Err(42);
   let unwrapped_err = result.unwrap_err();
   ```

1. `unwrap_or`: Unwraps the `Ok` value of the `Result`, returning the provided default value if the `Result` is `Err`.

   ```rs
   let default = 2;
   let x: Result<u32, &str> = Ok(9);
   assert_eq!(x.unwrap_or(default), 9);

   let x: Result<u32, &str> = Err("error");
   assert_eq!(x.unwrap_or(default), default);

   fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
       s.parse::<i32>()
   }

   let input = "42";
   let parsed = parse_number(input).unwrap_or(0);
   println!("Parsed value: {}", parsed);
   ```

1. `unwrap_or_else`: Similar to `unwrap_or`, but applies a function to the `Err` value and returns the new value produced by that function.

   ```rs
   fn count(x: &str) -> usize { x.len() }

   assert_eq!(Ok(2).unwrap_or_else(count), 2);
   assert_eq!(Err("foo").unwrap_or_else(count), 3);
   ```

1. `expect`: Unwraps the `Ok` value of the `Result`, with a custom panic message if the `Result` is `Err`.

   ```rs
   let result: Result<i32, &str> = Ok(42);
   let expected = result.expect("something went wrong");
   ```

1. `is_ok`: Returns `true` if the `Result` is `Ok`, `false` otherwise.

   ```rs
   let x: Result<i32, &str> = Ok(-3);
   assert_eq!(x.is_ok(), true);

   let x: Result<i32, &str> = Err("Some error message");
   assert_eq!(x.is_ok(), false);
   ```

1. `is_err`: Returns `true` if the `Result` is `Err`, `false` otherwise.

   ```rs
   let x: Result<i32, &str> = Ok(-3);
   assert_eq!(x.is_err(), false);

   let x: Result<i32, &str> = Err("Some error message");
   assert_eq!(x.is_err(), true);
   ```

1. `unwrap_unchecked`: This method is similar to `unwrap`, but it is marked as `unsafe` because it assumes that the Result is not an `Err` variant without checking.

   ```rs
   let result: Result<i32, &str> = Ok(42);
   let value = unsafe { result.unwrap_unchecked() };
   assert_eq!(value, 42);
   ```

1. `unwrap_or_default`: This method returns the inner value of an Ok variant if it exists, otherwise it returns the default value for that type.

   ```rs
   let result: Result<i32, &str> = Err("Oops");
   let default = 42;
   let value = result.unwrap_or_default(default);
   assert_eq!(value, 42);
   ```

1. `as_ref`: This method returns a reference to the inner value of the Result, allowing you to apply methods to it without taking ownership of the value.

   ```rs
   let result: Result<i32, &str> = Ok(42);
   let reference = result.as_ref().unwrap();
   assert_eq!(*reference, 42);
   ```

1. `as_mut`: This method returns a mutable reference to the inner value of the Result, allowing you to modify it if it is an Ok variant.

   ```rs
   let mut result: Result<i32, &str> = Ok(42);
   if let Ok(value) = result.as_mut() {
       *value = 0;
   }
   assert_eq!(result, Ok(0));
   ```

### Result Type Aliases

In Rust, you can define type aliases to make your code more concise and easier to read. You can also define type aliases for the Result type to make your code more generic and reusable.

```rs
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

This defines a new type alias `Result<T>` that is equivalent to `std::result::Result<T, Box<dyn std::error::Error>>`.

```rs
type CustomResult<T, E> = std::result::Result<T, CustomError<E>>;
```

This defines a new type alias `CustomResult<T, E>` that is equivalent to `std::result::Result<T, CustomError<E>>`.

### Printing Errors

There are several ways to print errors in Rust, including using the `println!` macro, the `eprintln!` macro, or the `format!` macro.

```rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "nonexistent.txt";
    let file = std::fs::File::open(file_name)?;

    Ok(())
}

fn handle_error(err: Box<dyn std::error::Error>) {
    println!("Error: {}", err);
    eprintln!("Error: {}", err);
    let error_message = format!("Error: {}", err);
    // do something with the error message
}
```

### Error Methods

In Rust, the `std::error::Error` trait defines a set of common methods that can be used to get information about errors. Some of the most commonly used methods are:

1. `fn description(&self) -> &str`: This method returns a short description of the error.
1. `fn cause(&self) -> Option<&dyn Error>`: This method returns the underlying cause of the error, if any.
1. `fn source(&self) -> Option<&(dyn Error + 'static)>`: This method returns the lower-level source of this error, if any.

```rs
use std::error::Error;
use std::io;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from("nonexistent.txt");
    let file = File::open(&file_path).map_err(|e| MyError::from((e, file_path.clone())))?;
    Ok(())
}

#[derive(Debug)]
struct MyError {
    source: Option<Box<dyn Error>>,
    path: Option<PathBuf>,
}

impl MyError {
    fn from((error: io::Error, path: PathBuf)) -> MyError {
        MyError {
            source: Some(Box::new(error)),
            path: Some(path),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error opening file")?;
        if let Some(path) = &self.path {
            write!(f, ": {:?}", path)?;
        }
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}
```

1. `fn display(&self) -> Display`: This method returns a human-readable description of the error.

```rs
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Custom Error: {}", self.message)
    }
}

fn main() -> Result<(), CustomError> {
    let err = CustomError {
        message: "Something went wrong".to_string(),
    };
    Err(err)
}
```

1. `fn downcast_ref<T>(&self) -> Option<&T>`: This method attempts to downcast the error to a specific type.

```rs
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Custom error: {}", self.message)
    }
}

impl Error for CustomError {}

fn do_something() -> Result<(), Box<dyn Error>> {
    let result = do_something_else()?;
    Ok(result)
}

fn do_something_else() -> Result<(), Box<dyn Error>> {
    Err(Box::new(CustomError {
        message: "oops".to_string(),
    }))
}

fn main() {
    let result = do_something();
    if let Err(err) = result {
        println!("Error: {}", err);
        if let Some(custom_error) = err.downcast_ref::<CustomError>() {
            println!("Custom error message: {}", custom_error.message);
        }
    }
}
```

1. `fn downcast_mut<T>(&mut self) -> Option<&mut T>`: This method attempts to downcast the error to a mutable reference of a specific type.

These methods allow developers to get more information about an error and provide better error messages to the user. It is also possible to implement custom error types that implement the `std::error::Error` trait and provide their own custom methods.

### Propagation of Errors

One of the essential features of Rust's error handling is the ability to propagate errors. This means that when a function returns a Result type, it can pass the error value back up the call stack, allowing the caller to handle the error instead of handling them at the point where they occurred. This is achieved using the `?` operator. When you use the `?` operator in a function that returns a `Result` type, the operator will return the value inside the `Ok` variant if it exists, or it will return the `Err` variant if an error occurred.

```rs
use std::fs::File;
use std::io::Read;

fn read_file_contents(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn print_file_contents(file_name: &str) {
    match read_file_contents(file_name) {
        Ok(contents) => println!("{}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
```

The `?` operator is used to propagate errors from the `File::open` and `file.read_to_string` functions.
