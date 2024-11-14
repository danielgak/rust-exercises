## Comands 

- **rustup** - toolchain installer
- **rustc** - compiles specificly
- **rustfmt** - formats
- **cargo new proj** - creates a project squeleton
- **cargo build** - builds the project in debug mode
- **cargo build** --release - builds the project as release
- **cargo run** - builds and run
- **cargo check** - check if compiles
- **cargo update** - recalculates Cargo.lock to bump minor versions

## Notes

### Syntax and scructure

- cargo projects requires you to store code in src/ and to have the Cargo.toml file as the project structure
- default build is a debug build, so the compiled entrypoint after a cargo build it's located in *target/debug/[project-name]s*
- `//` for comments
- mandatory semicolons
- `=` tells Rust to bind 
- in `String::new`, `::` indicates that `new` is an associated function of the `String` type
-  In `println!("x = {x} and y + 2 = {}", y + 2);` The `{}` it's the placeholder of the function param and `{x}` of the `x` variable

### Macros
- println!() calls a macro, different from println() that would be calling a function

### Variables

- In Rust variables are immutable by default, to specify mutability, use: let mut
- `&` indicates that *(for example)* the argument passing, is a reference. This gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- like variables, references are immutable by default

### Libraries

- Rust has a set of items in the std libary, that it brings to every program. This set it's the [prelude](https://doc.rust-lang.org/std/prelude/index.html).
- `String::new` is a function that returns a new instance of a `String`.  String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
- If we hadn’t imported the io library with use `std::io;` at the beginning of the program, we could still use the function by writing this function call as `std::io::stdin`. 
- Cargo uses [Semantic Versioning](https://semver.org/)
- Cargo uses [Crates.io](https://crates.io/) as the main registry
- When Cargo downloads dependencies, it grabbs other crates that your project dependency depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

### Enumeration

- an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
- The purpose of these `Result` types is to encode error-handling information. `Result`’s variants are `Ok` and `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

### Error handling

- If you don’t call expect, the program will compile, but you’ll get a warning.
> this `Result` may be an `Err` variant, which should be handled
- The right way to suppress the warning is to actually write error-handling code, but in our case we just want to crash this program when a problem occurs, so we can use `.expect`. 

### Crates

- When you build with `cargo build`. The project that you build is a **binary crate**, which is an executable. 
- A **library crate** its different from a **binary crate**. A **libary crate** contains code that is intended to be used in other programs and can’t be executed on its own.

### Cargo.lock

This is the mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the rand crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the Cargo.lock file the first time you run cargo build, so we now have this in the guessing_game directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

When you do want to update a crate, Cargo provides the command `update`, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. 