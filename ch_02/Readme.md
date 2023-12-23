# A Tour of Rust

Rust is a programming language that is designed to be fast, safe, and concurrent.

## Key Features

### 1\. **Memory Safety:**

- **Ownership:** Rust's ownership system prevents issues like dangling pointers and memory leaks by tracking resource ownership and enforcing strict borrowing rules.
- **Borrowing:** Allows simultaneous read-only access or exclusive mutable access to data, ensuring safe concurrency.

### 2\. **Concurrency without Data Races:**

- **Built-in Concurrency:** Rust's ownership model enables safe concurrent programming, ensuring threads cannot cause data races or access shared data unsafely.
- **Threads and Sync Primitives:** Provides thread support and synchronization primitives like mutexes and channels for safe data sharing.

### 3\. **Performance:**

- **Efficiency:** Rust delivers performance comparable to C/C++ while ensuring memory safety and preventing undefined behavior.
- **Compiler Optimizations:** Employs LLVM-based optimizations to generate highly optimized machine code.

### 4\. **Zero-Cost Abstractions:**

- **High-Level Abstractions:** Rust offers expressive, high-level features like iterators, pattern matching, and generics that are optimized to perform as efficiently as low-level code.

### 5\. **Predictable Behavior:**

- **Predictable Performance:** Emphasizes predictable performance characteristics, avoiding unexpected runtime behavior.
- **Compile-Time Safety Checks:** Many errors caught at compile time, ensuring code correctness before execution.

### 6\. **Focus on Safety and Reliability:**

- **No Null Pointers:** Rust's `Option` type prevents null pointer dereferences, enhancing safety.
- **Safe Concurrency:** Enforces thread safety and prevents common concurrency issues at compile time.

### 7\. **Developer Experience:**

- **Ease of Maintenance:** Facilitates easier debugging and modification, reducing risks in maintaining large codebases.
- **Strong Ecosystem:** Rich ecosystem with extensive libraries, fostering growth and adoption across diverse domains.

### 8\. **Zero-Overhead Principle:**

- **Efficient Abstractions:** High-level constructs and abstractions perform without introducing unnecessary runtime overhead, ensuring optimal performance without sacrificing safety.

Rust's combination of memory safety, concurrency support, performance, and developer-friendly features positions it as a robust language suitable for systems programming, high-performance applications, and reliable software development across various domains.

## Rustup and Cargo

### Rustup

Rustup is a command-line tool used for managing Rust versions and toolchains. Its key functionalities include:

1. **Installation and Version Management**:

   - Enables easy installation of the Rust programming language on various platforms.
   - Manages multiple Rust toolchains, allowing switching between stable, beta, and nightly versions.
   - Handles updates and manages the Rust compiler and associated tools.

1. **Managing Components**:

   - Allows adding or removing components like additional toolchains, targets, and documentation.

1. **Cross-Compilation**:

   - Facilitates cross-compiling Rust code for different target architectures or operating systems.

### Cargo

Rust's package manager and build system. It provides several features:

1. **Project Management**:

   - Initiates new projects with standardized project structures using `cargo new`.
   - Handles dependencies by managing crates (Rust packages) via `Cargo.toml`.

1. **Build Automation**:

   - Builds projects using `cargo build`, handling compilation, linking, and producing executable binaries.
   - Supports building and running tests with `cargo test`.
   - Facilitates creating optimized release builds using `cargo build --release`.

1. **Dependency Management**:

   - Manages project dependencies and their versions by specifying them in the `Cargo.toml` manifest file.
   - Downloads, updates, and builds dependencies automatically.

1. **Documentation Generation**:

   - Generates and hosts project documentation using cargo doc.
   - Supports Rust's documentation conventions, allowing clear and accessible documentation generation for projects.

To install rustup, you can follow the instructions on the Rust website: **<https://www.rust-lang.org/tools/install>**

Once rustup is installed, you can use it to install the latest stable version of the Rust compiler and the cargo tool by running the following command:

```shell
rustup install stable
```

You can also use `rustup` to switch between different versions of the Rust compiler and to install additional tools, such as Rustfmt (a tool for formatting Rust code) and Clippy (a linting tool for Rust). For example, to install Rustfmt, you can run the following command:

```shell
rustup component add rustfmt
```

To create a new Rust project with `cargo`, you can use the `cargo new` command. This command generates the basic files and directories needed for a new Rust project, including a `Cargo.toml` file that specifies the project's dependencies and build settings.

For example, to create a new Rust project called "my-project", you can run the following command:

```shell
cargo new my-project
```

The `Cargo.toml` file is the main configuration file for the project. It specifies the name, version, and dependencies of the project. The `src` directory contains the Rust source code for the project. The `main.rs` file is the entry point for the project, and it contains the main function that is run when the project is built and executed.

You can then navigate to the project directory and build the project with cargo by running the following command:

```shell
cargo build
```

This will compile the project and create an executable file in the target directory. To run the project, you can use the following command:

```shell
cargo run
```

Here is a summary of the `rustup` and `cargo` commands:

### `rustup` Commands

| Command                        | Description                                                                 |
| ------------------------------ | --------------------------------------------------------------------------- |
| `rustup install stable`        | Install the latest stable version of the Rust compiler and the `cargo` tool |
| `rustup component add rustfmt` | Install the Rustfmt tool for formatting Rust code                           |
| `rustup component add clippy`  | Install the Clippy linting tool for Rust                                    |
| `rustup update`                | Update Rust toolchains and associated tools to the latest versions          |
| `rustup default nightly`       | Set the default toolchain to the nightly version of Rust                    |
| `rustup target add <target>`   | Add a specific target to the Rust toolchain for cross-compilation           |
| `rustup toolchain list`        | List installed Rust toolchains and show the active toolchain                |
| `rustup self update`           | Update the `rustup` tool itself to the latest version                       |

### `cargo` Commands

| Command                | Description                                                                              |
| ---------------------- | ---------------------------------------------------------------------------------------- |
| `cargo new my-project` | Create a new Rust project with the name "my-project"                                     |
| `cargo build`          | Build the current project                                                                |
| `cargo run`            | Build and run the current project                                                        |
| `cargo test`           | Run the tests for the current project                                                    |
| `cargo doc`            | Generate documentation for the current project                                           |
| `cargo publish`        | Publish the current project to [crates.io](https://crates.io), the Rust package registry |
| `cargo clean`          | Clean the target directory, removing build artifacts                                     |
| `cargo check`          | Analyze code without building, performing type checks and catching errors                |
| `cargo fmt`            | Format Rust code according to specified style conventions                                |
| `cargo update`         | Update dependencies as per the versions specified in the `Cargo.toml` file               |
| `cargo bench`          | Run benchmarks to measure code performance and analyze execution time                    |

The `Cargo.toml` file is a configuration file used by Cargo, Rust's package manager and build system. It's located at the root of a Rust project and holds essential metadata and configuration settings for the project.

### Contents of `Cargo.toml`

1. **Project Metadata:**

   - **`[package]` Section:** Contains metadata about the project.
     - `name`: The name of the package.
     - `version`: The version of the package in semantic versioning format (`X.Y.Z`).
     - `authors`: Names or email addresses of the project authors.
     - `edition`: Specifies the Rust edition used in the project (`2021`).

2. **Dependencies:**

   - **`[dependencies]` Section:** Lists external crates (Rust packages) and their versions.
     - `crate_name`: Dependency crate name followed by its version or version constraints.
   - **`[dev-dependencies]` Section:** Lists crates used for development and testing purposes.

3. **Build Configuration:**

   - **`[build]` Section:** Allows specifying a build script or additional build settings.

### Example `Cargo.toml`

```toml
[package]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <email@example.com>"]
edition = "2021"

[dependencies]
somecrate = "1.2.3"

[dev-dependencies]
somecrate = "1.2.3"

[build]
# Additional build configurations, if needed
```

This file is vital for Cargo to manage project metadata, dependencies, build settings, and other project-specific configurations. It enables seamless project management, ensuring consistent builds, proper dependency resolution, and version control for Rust projects.

## Hello World Program

```rust
fn main() {
    println!("Hello, world!");
}
```

- The `fn main()` is the entry point of every Rust program. It is the first function that is executed when the program runs.

- The `println!` macro is used to print a message to the console. It takes a string as an argument and prints it to the console, followed by a newline.

- The `!` after println indicates that it is a macro, rather than a function. Macros are a way to generate code at compile-time, rather than running code at runtime. They are often used for tasks such as printing messages or creating repetitive code.

Overall, this program is a simple example of how to use Rust to print a message to the console.

Here is an explanation of the syntax used in this Rust program:

| Syntax Element    | Purpose                                                                                                                                       |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `fn`              | Defines a function in Rust.                                                                                                                   |
| `main`            | The name of the function. The `main` function is the entry point of every Rust program.                                                       |
| `()`              | Enclose the parameters of the function. In this case, the `main` function does not take any arguments.                                        |
| `{` and `}`       | Enclose the body of the function. The body of a function is the code that is executed when the function is called.                            |
| `println!`        | A macro that is used to print a message to the console. It takes a string as an argument and prints it to the console, followed by a newline. |
| `"Hello, world!"` | A string literal that represents the message to be printed. It is enclosed in double quotes.                                                  |

## GCD Euclidean Algorithm

```rust
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
```

This code defines a function called greatest common divisor (`GCD`) that takes two arguments: `n` and `m`, both of which are unsigned 64-bit integers (`u64`). The function returns a value of type `u64`, which is the `GCD` of `n` and `m`.

Here is a breakdown of the function into smaller parts, with a description of what each part does:

### Part 1: Check for zero values

```rust
assert!(n != 0 && m != 0);
```

This part checks if either `n` or `m` is zero, and if either is zero, it causes the program to panic. The `assert!` macro is used to perform this check. It will only execute if the condition (`n != 0 && m != 0)` is not met. This check is important because the GCD of two numbers is not defined if either number is zero.

### Part 2: Initialize loop

```rust
while m != 0 {
    // Loop body goes here
}
```

This part initializes a while loop that continues until `m` becomes zero. The loop condition is `m != 0`, which means the loop will continue as long as `m` is not equal to zero. The loop body, which contains the remaining steps of the function, goes between the curly braces (`{}`).

### Part 3: Swap values of n and m if necessary

```rust
if m < n {
    let t = m;
    m = n;
    n = t;
}
```

This part checks if `m` is less than `n`, and if it is, the values of `n` and `m` are swapped. This is done using a `let` statement to declare a temporary variable `t`, which is set to the value of `m`. The values of `m` and `n` are then swapped using the temporary variable. This step ensures that n is always the smaller of the two numbers.

### Part 4: Update value of m

```rust
m = m % n;
```

This part updates the value of m to be the remainder of `m` divided by `n`. It uses the modulo operator (`%`) to calculate the remainder. For example, if m is 7 and n is 3, then `m` would be updated to be 1 (the remainder of 7 divided by 3).

### Part 5: Return result

```rust
n
```

This is the final part of the function, and it simply returns `n` as the result. The GCD of `n` and `m` is the value of `n` when the loop terminates, because this is the largest number that can divide both `n` and m without leaving a remainder.

Here is an ultra-summary of the steps in the function
| Name | Description |
|---------|--------------------------------------------------------------------------------------------------|
| Check | Check if either `n` or `m` is zero and return zero if either is zero. |
| Loop | Initialize a loop that continues until `m` becomes zero. |
| Swap | Inside the loop, check if `m` is less than `n`. If it is, swap the values of `n` and `m`. |
| Update | Update the value of `m` to be the remainder of `m` divided by `n`. |
| Return | The loop ends when `m` becomes zero. At this point, `n` is returned as the result. |

This function has a time complexity of O(log(n)), which means that the number of steps required to compute the GCD grows at most logarithmically with the size of the input numbers. This makes the function efficient for calculating the GCD of large numbers.
