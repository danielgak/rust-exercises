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
- **cargo doc --open** - generates and open the docs for all your dependencies

## Notes

### Syntax and scructure

- cargo projects requires you to store code in src/ and to have the Cargo.toml file as the project structure
- default build is a debug build, so the compiled entrypoint after a cargo build it's located in *target/debug/[project-name]s*
- `//` for comments
- mandatory semicolons
- `=` tells Rust to bind 
- in `String::new`, `::` indicates that `new` is an associated function of the `String` type
-  In `println!("x = {x} and y + 2 = {}", y + 2);` The `{}` it's the placeholder of the function param and `{x}` of the `x` variable

### Variables, constants

- In Rust variables are immutable by default, to specify mutability, use: let mut
- `&` indicates that *(for example)* the argument passing, is a reference. This gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- like variables, references are immutable by default
- Shadowing allows you redifine the variables declared with `let`, 
- You can't sadow a variable declared with `let mut`, 
- Constants are inmutable, and using the `const` one must specify the type
- Rust’s naming convention for constants is to use all uppercase with underscores between words. 
- Constants are valid for the entire time a program runs, within the scope in which they were declared.
- Rust is a statically typed language, which means that it must know the types of all variables at compile time

### Scalars

- Rus has 4 types of scalars, integers, floating point, boolean, char
- **Integers** - *i8,u8, i32, u32, i64, u64, i16, u16, i128, u128, isize, usize*
- **Floating point** - *f32, f64*
- **Boolean** - *bool*
- **Char** - *char*, specified with signle quotes `'ℤ'` as opposed to string literals
- isize and usize depend on the architecture you are running on 32 or 64 bits
- singed integers has a range of -(2n - 1) to 2n - 1 - 1 inclusive.
- unsigned integer have a range of 0 to 2n - 1.
- **Decimal** - 98_222
- **Hex** - 0xff
- **Octal** - 0o77
- **Binary** - 0b1111_0000
- **Byte** - (u8 only) b'A'
- Numeric literals can use `_` as a visual separator to make the number easier to read, such as `1_000`
- In debug mode Rust includes checks for integer overflow that cause your program to panic at runtime
- Compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Relying on integer overflow’s wrapping behavior is considered an error.

### Compound Types
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
let o = tup.1;
println!("The value of y is: {y} and {o}");
```
- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. Tuples and Arrays.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- You can destructurue a compound type
- Access a tuple element directly by using a period `.`
```rust
let: [i32; 5] a = [1, 2, 3, 4, 5];
let b = [3; 5]; // [3, 3, 3, 3, 3]
let second = a[1];
```
- Arrays are useful when you want your data allocated on the stack rather than the heap

### Ranges

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

- Expresed like `start..=end` is inclusive on the lower and upper bounds, so we need to specify `1..=100` to request a number between 1 and 100.

### Matches

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

- A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
- Patterns and the match construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. 

### Macros

- println!() calls a macro, different from println() that would be calling a function

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
 `Result` may be an `Err` variant, which should be handled.
- The right way to suppress the warning is to actually write error-handling code, but in our case we just want to crash this program when a problem occurs, so we can use `.expect`. 
- The underscore, `_`, is a catchall value; `Err(_) => continue` we’re saying we want to match all Err values, no matter what information they have inside them.
- Panicking is when a program exits with an error; 

### Crates

- When you build with `cargo build`. The project that you build is a **binary crate**, which is an executable. 
- A **library crate** its different from a **binary crate**. A **libary crate** contains code that is intended to be used in other programs and can’t be executed on its own.

### Cargo.lock

This is the mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the rand crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the Cargo.lock file the first time you run cargo build, so we now have this in the guessing_game directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

When you do want to update a crate, Cargo provides the command `update`, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. 