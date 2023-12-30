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
