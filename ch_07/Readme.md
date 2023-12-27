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
