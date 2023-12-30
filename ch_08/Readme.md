# Crates and Modules

In Rust, programs are built using crates, which serve as self-contained units. Each crate comprises all the necessary components for a particular library or executable, encompassing source code, tests, examples, tools, configurations, and other related elements.

A module is a way of organizing code within a crate. Modules allow you to group related code together, control visibility, and manage namespaces.

## Crates

A crate is a compilation unit that produces a library or executable binary.

To gain a comprehensive understanding of crates and their interactions, a practical approach involves using Cargo. By running `cargo build` along with the `--verbose` flag in an existing project that relies on dependencies, you can observe how crates function and interconnect.

This command provides detailed insights into the build process, displaying information about how crates are fetched, compiled, and linked together. The verbosity flag (`--verbose`) offers an in-depth look into the dependencies and their interactions, aiding in the comprehension of how these crates collaborate within the project.

In the context of Rust and Cargo, "dependencies" refer to the other crates or external code that a particular project relies on. These are essentially the crates that our project depends on or uses to fulfill specific functionalities or incorporate certain features.

These crates, serving as dependencies, are typically sourced from crates.io, the Rust community's platform dedicated to open-source crates.

On crates.io, each crate's page displays essential information about the crate. This includes the contents of its README.md file, links to its documentation and source code repositories, and a specific line of configuration that you can copy and add to your project's Cargo.toml file.

The version numbers showcased on these pages represent the latest versions available for the crate at the time of our program's development. These versions serve as references for developers seeking to incorporate these crates into their projects, ensuring they are using the most recent and potentially improved functionalities and fixes offered by these crates.

When you execute `cargo build`, Cargo initiates the process by fetching the source code of the specified crate versions from crates.io. Once obtained, Cargo proceeds by parsing the Cargo.toml files within these crates.

Following this, Cargo identifies and downloads the dependencies declared within these crates' configurations. This process of obtaining dependencies occurs recursively, meaning Cargo continues to delve into each dependency's own specifications, fetching their dependencies as well, creating a cascading chain of downloads until all required components are acquired.

The dependencies that are fetched as a result of this recursive process are termed "transitive dependencies".

The amalgamation of all these interlinked dependency relationships forms a comprehensive structure that informs Cargo about the entirety of necessary crates to build, as well as the precise sequence in which these crates should be constructed. This entire ensemble of interdependent relationships, detailing what crates are required and the hierarchical order in which they must be assembled, constitutes what we refer to as the "dependency graph" of the crate.

For each crate present in the project's dependency graph, Cargo invokes the Rust compiler, rustc.

When compiling libraries, Cargo employs the --crate-type lib option. This directive instructs rustc not to search for a `main()` function but rather to generate an .rlib file. This .rlib file comprises compiled code, specifically designed for utilization in producing binaries or creating additional .rlib files. Essentially, it serves as a library that can be utilized in building other components of the project.

When Cargo engages in compiling a program, it employs the --crate-type bin option. This directive prompts the Rust compiler, rustc, to produce an output that's specifically a binary executable file that can be directly run on the designated platform without any further compilation steps, providing a standalone program ready for execution.

With each rustc command, Cargo includes --extern options, specifying the filenames of the libraries that the crate will utilize. This way, when rustc encounters a line of code like use `image::png::PNGEncoder`, it can identify that image is the name of another crate. Thanks to Cargo's assistance, rustc knows where to locate the compiled crate on the disk.

The Rust compiler necessitates access to these .rlib files because they contain the compiled code of the library. Rust statically links this code into the final executable. Additionally, the .rlib files encompass essential type information, enabling Rust to verify that the features from the library employed in our code indeed exist in the crate and are used correctly. Furthermore, these files contain a snapshot of the crate's public inline functions, generics, and macros. Some features cannot be entirely compiled into machine code until Rust analyzes how we employ them in our code.

One notable option is `cargo build --release`, which generates an optimized build. Release builds execute faster but take longer to compile. They bypass checks for integer overflow, omit `debug_assert!()` assertions, and generally produce less reliable stack traces in case of a panic.

### Editions

Rust takes pride in its robust compatibility guarantees. Code that compiled on Rust 1.0 is assured to compile equally well on Rust 1.50 or any subsequent version like Rust 1.900, if released. However, proposals for significant language extensions sometimes arise, presenting challenges when implementing changes that might render older code incompatible. For instance, adopting a syntax for asynchronous programming with async and await as keywords could break existing code using these identifiers as variable names.

To address this challenge without disrupting existing codebases, Rust introduced the concept of "editions." Each edition of Rust represents a distinct set of language features and rules. The 2015 edition, for instance, maintains compatibility with Rust 1.0. Subsequently, the 2018 edition introduced changes like redefining async and await as keywords, refining the module system, and implementing other language modifications that are incompatible with the 2015 edition.

In the Cargo.toml file's \[package\] section, crates indicate the Rust edition they're written in via a line like `edition = "2018"`. If this line is absent, the 2015 edition is assumed, ensuring backward compatibility. However, to utilize features specific to newer editions, such as asynchronous functions or the updated module system, the `edition = "2018"` (or a more recent edition) declaration in the Cargo.toml file becomes necessary.

Crucially, Rust assures that the compiler accepts all existing editions of the language, allowing programs to seamlessly integrate crates written in different editions. A crate's edition solely influences how its source code is interpreted; once compiled, edition distinctions are irrelevant. This eliminates the necessity of updating old crates solely to engage with the modern Rust ecosystem or forcing a crate to adhere to an older edition to accommodate users. Edition changes are only required when incorporating new language features into your code.

The Rust project doesn't release editions annually; they're introduced when deemed necessary. It's generally recommended to utilize the latest edition, especially for new code, as `cargo new` defaults to creating projects on the latest edition. Additionally, for crates written in older Rust editions, the `cargo fix` command might assist in automatically upgrading the code to a newer edition. The Rust Edition Guide offers comprehensive coverage of edition changes and details the `cargo fix` command.

### Build profiles

Build profiles allow developers to configure and manage different build settings for various scenarios or purposes within their Cargo.toml file. These profiles offer a way to specify distinct configurations tailored to different use cases, such as development, testing, or production.

There are several predefined build profiles:

- **Default Profile**: This profile is utilized when no specific profile is chosen. It generally emphasizes speed and ease of debugging, ideal for day-to-day development work.

- **Release Profile**: The release profile is optimized for producing efficient and optimized code for deployment in production. It typically includes compiler optimizations and excludes debug information to enhance performance.

- **Custom Profiles**: Developers can create additional profiles as needed, allowing them to define unique configurations for specific purposes or environments. For instance, a custom profile optimized for testing might retain certain debugging features without compromising performance significantly.

Here's a table mapping the Cargo.toml configuration settings sections to their respective command line usage:

| Command                 | Cargo.toml Section  |
| ----------------------- | ------------------- |
| `cargo build`           | `[profile.dev]`     |
| `cargo build --release` | `[profile.release]` |
| `cargo test`            | `[profile.test]`    |

These commands correspond to specific sections within the Cargo.toml file, enabling developers to specify different settings and configurations for various build scenarios. For instance, `[profile.dev]` is used to define settings for development builds, `[profile.release]` for optimized release builds, and `[profile.test]` for testing configurations.

Typically, default build settings work well for most scenarios. However, there's an exception when using profilers, tools that track where a program spends its CPU time. To gather comprehensive data from a profiler, both optimizations (commonly enabled in release builds) and debug symbols (usually present in debug builds) are necessary. Combining these can be achieved by modifying your Cargo.toml file:

```toml
[profile.release]
debug = true # enable debug symbols in release builds
```

This configuration controls the inclusion of debug symbols (-g option) in the Rust compiler (rustc). With this setup, executing `cargo build --release` results in a binary containing both debug symbols and the optimizations typically associated with release builds. The optimization settings remain unaffected by the addition of debug symbols.

## Modules

Modules are organizational units that help structure code within a crate. They allow developers to group related functionalities, types, and traits together, enhancing code organization and readability. Modules aid in managing code complexity by breaking it into smaller, more manageable parts.

To create a module in Rust, you can use the `mod` keyword:

```rs
mod my_module {
    // module code goes here
}
```

Key aspects of modules include:

1. **Encapsulation:** Modules encapsulate code, acting as a boundary that defines the visibility of items (functions, structs, enums, etc.). Items can be marked as public or private within a module, controlling their accessibility to code outside that module.
2. **Namespace:** Modules provide a namespace, preventing naming collisions by allowing items with the same name to coexist within different modules.
3. **Nesting:** Modules can be nested within each other, allowing for hierarchical organization of code. This nested structure is represented using the `mod` keyword.
4. **File Organization:** Rust follows a convention where module structure often aligns with the file system structure. Each module may reside in its own file or be grouped together in a single file, aiding in code navigation and maintenance.

### Nested modules

Nested modules refer to the ability to organize modules within other modules, creating a hierarchical structure within a crate.

```rs
// Define an outer module named 'math'
mod math {
    // Declare a submodule named 'operations' within the 'math' module
    pub mod operations {
        // Define functions within the 'operations' submodule
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
    }

    // Declare another submodule named 'constants' within the 'math' module
    pub mod constants {
        // Define constant values within the 'constants' submodule
        pub const PI: f64 = 3.14159;
        pub const E: f64 = 2.71828;
    }
}

fn main() {
    // Access functions from the 'operations' submodule in the 'math' module
    let result_add = math::operations::add(10, 5);
    let result_subtract = math::operations::subtract(20, 8);

    println!("Addition result: {}", result_add);
    println!("Subtraction result: {}", result_subtract);

    // Access constants from the 'constants' submodule in the 'math' module
    println!("Value of PI: {}", math::constants::PI);
    println!("Value of E: {}", math::constants::E);
}
```

### Visibility

By default, items (functions, structs, etc.) in a module are not visible outside of that module. Rust offers granular control over the visibility of items within modules using different pub modifiers.

- `pub`: Marks an item (function, struct, enum, etc.) as public, allowing it to be accessed from outside its module or crate.

  ```rs
  mod my_module {
      // This function is not visible outside of the module
      fn my_private_function() {}

      // This function is visible outside of the module
      pub fn my_public_function() {}
  }
  ```

- `pub(crate)`: Makes an item visible only within the current crate. It restricts visibility to the current crate but allows access from any module or function within that crate.

  ```rs
  // Define a module named 'my_module' in the same crate
  mod my_module {
      // Define a function accessible within the current crate
      pub(crate) fn internal_function() {
          println!("This function is accessible within the crate");
      }
  }

  fn main() {
      // Access the function from the same crate
      my_module::internal_function();
  }
  ```

- `pub(super)`: Specifies that an item is visible to the parent module only. It restricts visibility to the immediate parent module and hides it from sibling or descendant modules.

  ```rs
  mod parent_module {
      // Define a submodule 'sub_module'
      mod sub_module {
          // Define a function visible only to the parent module
          pub(super) fn restricted_function() {
              println!("This function is accessible to the parent module only");
          }
      }

      // Attempting to access 'restricted_function' from outside 'sub_module' will result in an error
      // sub_module::restricted_function(); // Uncommenting this line will result in a compile-time error
  }
  ```

- `pub(in <path>)`: Sets visibility for an item within a specific module path and its descendants. This form of visibility restricts access to a particular module and its nested modules, allowing the item to be accessed only from that specific path.

  ```rs
  mod my_module {
      // Define a submodule 'nested'
      mod nested {
          // Define a function accessible within the 'my_module' and 'nested' modules
          pub(in crate::my_module::nested) fn specific_function() {
              println!("This function is accessible within 'my_module::nested' and its descendants");
          }
      }

      // Access 'specific_function' from 'my_module' or 'nested' modules
      nested::specific_function();
  }
  ```

### Modules in Separate Files

Modules can be organized across separate files, aligning with the module structure to maintain a clean and manageable codebase. This approach helps in structuring larger projects and facilitates better organization and readability.

Here's an example demonstrating how modules can span multiple files:

Consider a project structure like this:

```less
math_crate/
│
├── Cargo.toml          // Package manifest
└── src/
    ├── main.rs         // Entry point (optional for binaries)
    └── lib.rs          // Entry point for the library crate
    └── math/
        ├── mod.rs      // Module declaration file
        ├── operations.rs // File containing operation functions
        └── constants.rs  // File containing constant values
```

Contents of `Cargo.toml`:

```toml
[package]
name = "math_crate"
version = "0.1.0"
edition = "2021"

[lib]
name = "math_lib"
path = "src/lib.rs"

[[bin]]
name = "main"
path = "src/main.rs"
```

The `main.rs` file is the entry point of the application, and within the `math` directory, there are three files:

1. `mod.rs`: This file declares the module structure.

   ```rs
   // Declare the module 'math' and its contents
   pub mod operations;
   pub mod constants;
   ```

2. `operations.rs`: This file contains functions related to operations.

   ```rs
   // Define functions within the 'operations' module
   pub fn add(a: i32, b: i32) -> i32 {
       a + b
   }

   pub fn subtract(a: i32, b: i32) -> i32 {
       a - b
   }
   ```

3. `constants.rs`: This file contains constant values.

   ```rs
   // Define constant values within the 'constants' module
   pub const PI: f64 = 3.14159;
   pub const E: f64 = 2.71828;
   ```

In the `lib.rs` file:

```rs
// Declare the module structure within the crate
pub mod math;
```

In `main.rs`, or any other file within the same crate:

```rs
// Access the 'math' module
use math_crate::math;

fn main() {
    // Use functions and constants from the 'math' module
    println!("Addition result: {}", math::operations::add(10, 5));
    println!("Value of PI: {}", math::constants::PI);
}
```

This structure allows developers to organize code within a module across multiple files, keeping related functionalities segregated for better code management and readability in larger projects. The `mod.rs` file acts as the entry point to declare the module's structure, and other files contain the actual implementations for various components of the module.

### Paths and Imports

#### Paths

- **Absolute Paths**: Start from the crate root using the crate name or from a crate root using `crate`.
- **Relative Paths**: Start from the current module using `self`, `super`, or a module's identifier.

#### Imports

- **Use Declarations**: Bring items (functions, structs, etc.) into scope, reducing the need for long path references.
- **use Keyword**: Enables importing items from a path into the current scope.

The `::` operator (path separator) is used to access features within modules or crates. It's employed to reference specific items, such as functions or constants, within the specified module or crate.

For instance, in the code snippet:

```rs
use std::mem;

if s1 > s2 {
    mem::swap(&mut s1, &mut s2);
}
```

Here, `std::mem` refers to the `mem` module within the `std` crate, and `mem::swap` accesses the `swap` function within the `mem` module.

Importing with the `use` declaration allows creating local aliases for modules or items, making it more convenient to access them within the scope of the block or module where the import is specified.

Additionally, the `use` declaration can import multiple items or modules at once, either by specifying each individually or using the shorthand syntax:

```rs
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::prelude::*;
```

Using `as` with `use` allows importing an item while giving it a different name locally, as demonstrated in:

```rs
use std::io::Result as IOResult;
```

Modules don't inherently inherit names from their parent modules. Instead, each module begins with an empty scope and must explicitly import the names it requires. Paths within a module are by default relative to that module's context.

The keyword `self` is also used as a synonym for the current module. It represents the module itself and allows referencing items within the same module without specifying the full path. This facilitates shorter and more concise references to items within the current module.

```rs
// File: my_module.rs

// Define a submodule named 'sub_module'
mod sub_module {
    pub fn hello() {
        println!("Hello from sub_module!");
    }
}

// The current module
mod current_module {
    // Using `self` to reference the current module
    pub fn greet() {
        println!("Greetings from the current module!");
    }

    // Using relative paths to access items in the same module
    pub fn perform_greeting() {
        self::greet(); // Accessing 'greet' function using `self`
        super::sub_module::hello(); // Accessing 'hello' function from the sibling module
    }
}
```