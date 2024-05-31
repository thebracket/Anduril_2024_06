# What did Cargo do?

Cargo has created several files in your project folder:

|Filename|Description|
|--|--|
|hello_world/Cargo.toml|The build manifest. Equivalent to a CMake file or Makefile.|
|hello_world/src/|The source directory.|
|hello_world/src/main.rs|The main source code file. Every executable needs a `main.rs` file (libraries have a `lib.rs`)---you can override this, but it's a good default.|

### Cargo.toml

Rust has created a `Cargo.toml` file for you:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

* The `[package]` section defines the program itself. 
    * The `name` will be the name of the emitted executable (with `.exe` on the end for Windows). 
    * `version` uses semantic versioning. When referring to versions, "0" is special - every bump counts as a release. Once you hit 1.0, the dependency checker is a bit more strict. We'll talk about that later.
    * `edition` tells the Rust compiler which edition of the language you are using. Rust promises not to break language compatibility except when the edition number increases (roughly every 2-4 years). `rustc` retains compatibility with previous editions, unless a dangerous security reason appeared to remove something. This is designed to avoid the C++ difficulty of "we can never take away features" and "we can never change the interface".
* The `dependencies` section determines dependent packages to download. We'll worry about that later.

### main.rs

The `main.rs` file is a basic "Hello, world!" program:

```rust
fn main() {
    println!("Hello, world!");
}
```

If you've never seen Rust before, it might be a little confusing.

* `fn` is "function". Unlike C++, it doesn't specify the return type---just that it is a function.
* `main` is the name of the function. `main` is special, just like C++ --- it's the default invocation point for an executable program.
* `println!` has an exclamation mark, indicating that its a *macro*. Formatting strings is a pretty big job---see the C++20 format system! Rust's formatting system uses the macro system to allow for extreme flexibility for parameters. It's very powerful, but it's also a poor example for the first thing you see because macros are not an introductory topic.
