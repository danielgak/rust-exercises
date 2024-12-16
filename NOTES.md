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
- Rust's naming convention for constants is to use all uppercase with underscores between words. 
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
- Compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Relying on integer overflow's wrapping behavior is considered an error.

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
- if the accesed index is greater than or equal to the length, Rust will panic

### Ranges

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

- Expresed like `start..=end` is inclusive on the lower and upper bounds, so we need to specify `1..=100` to request a number between 1 and 100.

### Functions 

- `fn` Rust code uses snake case as the conventional style for function and variable names
- Rust doesn't care where you define your functions
- You must declare the type of each parameter on function signatures
- You must specify function's return eg.`-> i32`
- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value. 
```rust
fn main() {
    let y = 6;  // the 6 in the statement, let y = 6 is an expression
    let x = (let y = 6); // this can't be done as the assignment can only be done into an expresionn
}
```
- **Expressions** can be part of **statements**
- A new scope block created with curly brackets is an **expression**
```rust
fn main() {
    let y = { // expresion bc x + 1 does not have a semicolon
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

### Control flow

- condition in this code must be a bool
- Blocks of code associated with the conditions in if expressions are sometimes also called arms (as in Matchers)
- Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable. Eg. `let number = if condition { 5 } else { 6 };`
- Rust can detect that this: `let number = if condition { 5 } else { "six" };` is an error, as number inference is not possible.s
- `if`, `else` and `else if` are valid expresions.
- `loop`, `while`, and `for` are available.
- break can return a value in a loop. `break` only exits the current loop, `return` always exits the current function.
```rust
let mut number = loop { // break can return
    counter += 1; 
    if counter == 10 {
        }
        break counter * 2;
};

while number != 0 { // while
    number -= 1;
}

let a = [10, 20, 30, 40, 50]; // for elements 
for element in a {
    println!("the value is: {element}");
}

for number in (1..4).rev() { // for 
    println!("{number}!");
}

```
- nested loops can be broken and continued from any point inside of the loop, even in the innermost loop. Label the parent loop, and reference it in the break.
```rust
'counting_up: loop {
    // something
    loop {
        // something
        if count == 2 {
            break 'counting_up;
        }
    }
}
```


### Macros

- println!() calls a macro, different from println() that would be calling a function

### Libraries

- Rust has a set of items in the std libary, that it brings to every program. This set it's the [prelude](https://doc.rust-lang.org/std/prelude/index.html).
- `String::new` is a function that returns a new instance of a `String`.  String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
- If we hadn't imported the io library with use `std::io;` at the beginning of the program, we could still use the function by writing this function call as `std::io::stdin`. 
- Cargo uses [Semantic Versioning](https://semver.org/)
- Cargo uses [Crates.io](https://crates.io/) as the main registry
- When Cargo downloads dependencies, it grabbs other crates that your project dependency depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

### Enumeration

- an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
- The purpose of these `Result` types is to encode error-handling information. `Result`’s variants are `Ok` and `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

### The Stack and the Heap

The stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.

- The stack stores values in the order it gets them and removes the values in the opposite order.  Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
- The heap is less organized. To stroe a value in the heap, the memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. *(allocating)*
- Pushing to the stack is faster. Allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. re. Contemporary processors are faster if they jump around less in memory (stack).
- When code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
- `string` literals are treated as inmutable hardcoded values so then can be stored in the stack. `String` is the mutable datastructure so it's stored in the heap.
- `String::from` requests the memory it needs from the memory allocator at runtime, but the droping it's managed by **rust ownership**
- languages with **garbage collection** has a dedicated proeccess that keeps track of and cleans up memory that isn’t being used anymore
- in languages without garbage collection, memory removal it's done manually thogh `allocate` and `free`
- when pointers are shared, there exist a known issue as `the double free error` that can lead memory corruption, which can potentially lead to security vulnerabilities. This is also avoided by **rust ownership**

### Ownership

Rust manages memory is managed through a system of ownership *(data cleaning up, duplication etc)*. There are three core rules:
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
```rust
let s1 = String::from("hello"); // pointer, capacity and len is stored in the stack, the rest is on the heap
let s2 = s1; // s1 is no longer valid, s2 will call drop once it leaves the scope
```
- Rust does not have the concept of shallowing copying, instead it **moves**
- Rust will never automatically create “deep” copies of your data. 
```rust
let s1 = String::from("hello"); 
let s2 = s1.clone(); // this is a deep copy, the heap data gets duplicated

let x = 5;
let y = x; // integer implements copy trait, so it does not invalidate
```
- Data types with known size at compile time are stored entirely on the stack, so no difference between shallow and deep copying here. This means that *integers*, 
*booleans*, *floats*, *chars* and *tuples* implement the Copy trait by default, meaning that they always copy and do not invalidate
- using a function applies same ownership rules as on variable assignment
```rust
let s = String::from("hello");  // s comes into scope
takes_ownership(s); // and owneship is given to the function (moved) so the pointer it's no longer valid here

let s2 = String::from("hello2");
let s3 = takes_and_gives_back(s2); // here you move away the scope, and take it back
```
- This design choice makes that invoking functions and reassining is a very common thing, and the tuple plays nicely into this allowing to return multiple values
- A `reference` `&` in Rust is like a pointer, an address we can follow to access the data. That data is owned by some other variable. 
- Unlike a traditional `pointer`, a `reference` is guaranteed to point to a valid value of a particular type for the life of that reference.
- The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`
```rust
let s1 = String::from("hello");
let len = calculate_length(&s1); // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
// But because it does own it, the value is not dropped
```
- We call the action of creating a reference **borrowing**. 
- Just as variables are immutable by default, so are references: you can't modify a borrowed var
```rust
let mut s = String::from("hello");
change(&mut s);

fn change(some_string: &mut String) {}
```
- a `mutable reference` can be modified
```rust 
let mut s = String::from("hello");

let r1 = &mut s;
// let r2 = &mut s; only one mut borrow!
// ... r's used
```
- Mutable references have one big restriction: if you have a mutable reference to a value, you can't have no other references to that value. The benefit is that Rust can prevent data races at compile time.
- Curly brackets can be used to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```
- A mutable reference while we have an immutable one to the same value, its not allowed either
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // problemo
println!("{r1} and {r2}"); // these 3 reference can't coexist
```
- Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used
``` rust 
let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // now, no problem! rust knows that r1 and r2 no longer a re used
println!("{r3}");
```
-  In Rust the compiler guarantees that references will never be dangling references

And by summary:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

### Data racing

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.


### Slices

- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
```rust
let s = String::from("hello world");

let hello = &s[0..5]; // same as &s[..5]
let world = &s[6..11]; //a pointer to the same date but sarting from a different position

let s2 = "hello world"; // this string literal, stores in the binary
// type of s here is &str, so its inmutable
````
- slices also works with any type of arrays
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

### Vectors
- // TODO:

### Strings

- Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.
- Two basic types:
    - **string slices** are references to some UTF-8 encoded string data stored in the heap
    - **string literals** are stored in the program’s binary
- Many of the same operations available with `Vec<T>` are available with `String` as well because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
- UTF 8 allows any properly encoded data
- `let hello = String::from("Hola");` will have len will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of these letters takes one byte when encoded in UTF-8.
- `let hello = String::from("Здравствуйте");` will have len 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage.
- Accessing individual characters in a string by referencing them by index, is not allowed in Rust `eg. str[0]`. because for the example before, `&"Здравствуйте"[0]` the letter `З` get represented in 2 values, so to avoid errors related with extracting meaningless byte values, Rust does not allow it.
- when talking about UTF-8, there are 3 relevant ways to look at from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters). With the Hindi word **नमस्ते** as an example let see it in 3 views:
    - `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]` are the bytes, 18 bytes in total
    - `['न', 'म', 'स', '्', 'त', 'े']` are characters, there are 6, but the 4th and 6th do not make sense on their own
    - `["न", "म", "स्", "ते"]` are  grapheme clusters, what a person would call the four letters that make up the Hindi word
- slicing *"Здравствуйте"* on [0..4], would give you *Зд*, but slicing it [0..1] will result in a panic. Be carefull when creating string slices with ranges, because doing so can crash your program.
- methods like `contains` and `replace` are usefull tools to avoid most of this nonAscii situations

### Collections

- Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, so they can grow and shrink more freely
- There are three type of collections `vector`, `string` and `hasmap`. A `vector` allows you to store a variable number of values next to each other.
- A `string` is a a collection of characters. 
- A `hash map` allows you to associate a value with a specific key, it's a particular implementation from the map data stucture. 
- `Vec<T>` allow you to store more than one value in a single data structure that puts all the values next to each other in memory
- Vectors can only store values that are of the same type. The variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum
- `HashMap<K, V>` stores a mapping of keys of type *K* to values of type *V* using a hashing function
- Just like vectors, hash maps store their data on the heap

### Structs

- you can use field init shorthand eg. `User { email, name: "something" }`
- `..` is the struct update syntax, similars as typescript but does not "deconstruct" already provided values
```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1 // this has email, but the one used is above
    // as this is a =, it moves the value! so user1 is no longer valid
}; // note that the "scruct User" owns the email attribute
```
- you can create stucture tuples also, eg. `struct Color(i32, i32, i32);`
- you can create unitlike structs without data eg. `struct AlwaysEqual;` similar to empty tuples `()`
- Unit-like sturcts can be used when you need to implement a trait on some type, but don’t have any data that you want to store in the type itself
- To print structs, you must implement the `Display` trait
- To debug structs, you can print them with `{:?}` notation adding `#[derive(Debug)]` statement into the struct definition, or with `dbg!` to output more information
- first parameter in a impl block when method definition eg `fn area(&self)` the self is a shortcut to `self: &Self`.
- methods can take ownership of self, borrow self immutably, or borrow self mutably, just as they can any other parameter.
- methods and attributes can be named the same
- Rust does automatic referencing and dereferencing. When you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method.
```rust
p1.distance(&p2); // both are the same 
(&p1).distance(&p2); 
```
- in the impl scope, `Self` is a keyword used to refer to the subject of the impl block
- Asociated functions are defined in the impl and do not have self as their first parameter. eg `Rectangle::square(2)`. They are often used as constructors
- Struct is allowed to have multiple impl blocks

### Matches

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

- A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm's pattern. 
- Patterns and the match construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. 

### Enums

- a custom data type, eg. `IpAddrKind::V4`
- Rust allows puting data directly into each enum variant, eg:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
```
- Rust does not have `null`, because in languages with null, variables can always be in one of two states: null or not-null.
- The concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.
- `Option<T>` enum encodes a value that could be something or it could be nothing. Containing always `Some` and `None`.
```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```
- Or even go crazier with the times in it
```rust
enum Message {
    Quit, //  no data
    Move { x: i32, y: i32 }, // named fields
    Write(String),
    ChangeColor(i32, i32, i32), // tuple
}
```
- Defining an enum with variants such as before, it's equivalent to have 3 different structs but grouped together under the Message type.
- `match` allows you to compare a value against a series of patterns, that can be values, variable names, wildcards, etc.
- the `arms` in pattern of the match, must cover all cases of the enum that you are matching against
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // catch all
}
```
- use `_ => reroll()` or `_ => ()` for example as a catch all pattern where the `_` is telling rust that the value wont be used
- with an if let expresion you can achieve same behaviour in some cases, less bervose
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```


### Error handling

- If you don't call expect, the program will compile, but you'll get a warning.
 `Result` may be an `Err` variant, which should be handled.
- The right way to suppress the warning is to actually write error-handling code, but in our case we just want to crash this program when a problem occurs, so we can use `.expect`. 
- The underscore, `_`, is a catchall value; `Err(_) => continue` we're saying we want to match all Err values, no matter what information they have inside them.
- Panicking is when a program exits with an error; 

### Cargo.lock

This is the mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the rand crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the Cargo.lock file the first time you run cargo build, so we now have this in the guessing_game directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

When you do want to update a crate, Cargo provides the command `update`, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. 

### Packages, Crates, Modules and Paths
- A crate is the smallest amount of code that the Rust compiler considers at a time, it can come in two forms: a *binary crate* or a *library crate*
- When you build with `cargo build`. The project that you build is a *binary crate*, which is an executable. *Binary crates* are programs that you compile to an executable that you can run, such as a command-line program or a server, and each one must have a function called `main` as the "entrypoint" of the execution 
- A **libary crate** contains code that is intended to be used in other programs and can't be executed on its own. Library crates don’t have a `main` function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. 
- the root crate it's the source file from witch Rust starts the compilation to create the root module of your crate
- Most of the time when Rustaceans say “crate”, they mean library crate
- A package is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file, that describe how to build those crates.
- A package can contain multiple binary crates and optionally one library crate. When the package grows, you can extract parts into separate crates as independent dependencies. For very large projects comprising a set of interrelated packages that evolve together, cargo has **workspaces**
- Cargo follows a convention that *src/main.rs* is the crate root of a binary crate with the same name as the package.
- Cargo knows that if the package directory contains *src/lib.rs*, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
- [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)
- Modules helps control privacy items because in a module everything is private by default.
- *src/main.rs* and *src/lib.rs* are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the **module tree**, eg `crate:front_of_house:hosting:add_to_waitlist()`
- An **absolute path** is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal *crate*, the eg. `crate:front_of_house:hosting:add_to_waitlist()` it's an absolute path within the project. Like using `/` in the filesystem.
- A **relative path** starts from the current module and uses self, super, or an identifier in the current module.
- Child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined
- privacy rules apply to structs, enums, functions, methods and modules
- `use` can bing functions into scope in an idiomatic without needing to specify the paths eg `use hosting::remove_from_waitlist` to invoque `remove_from_waitlist()` directly
- rust does not allow brining two fucntions into scope with the same naming
- the convention is to use the whole path when brining dependendencies into scope
- when using `as` you can specify an alias for the brought scope
- when we bring a name into scope with the `use` keyword, the name available in the new scope is private. if you want to export this, `pub use`
- To use an external crate, list it in your package’s Cargo.toml file and `use` to bring items from their crates into scope.
- **std** library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include **std**.
- `mod` **is not an “include”** operation that you may have seen in other programming languages
- child sub modules, can access private parent elements
- the directories and files more closely match the module tree
- For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
    - src/front_of_house.rs
    - src/front_of_house/mod.rs (older style, still supported path)
- For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:
    - src/front_of_house/hosting.rs (what we covered)
    - src/front_of_house/hosting/mod.rs (older style, still supported path)