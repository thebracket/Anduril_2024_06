# Collaborative: Create a C Library

> I was a bit nervous about asking a class with mixed platforms to all have a working C compiler in the same toolchain as Rust. Most of you have it - the runtimes generally install it. But its possible we'll be chasing some installation issues as we work through this.

Let's build a *tiny* C library to get started. Make a new Rust project, and in the project's main source add a file named `mylib.c`:

```c
// A really simple function that doubles a number
int double_int(int x) {
    return x * 2;
}
```

Now open your `Cargo.toml` file, and we're going to add a `build dependency` (a dependency that is only loaded during build and never reaches the final binary):

```toml
[package]
name = "c_to_rust"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]

[build-dependencies]
cc = "1"
```

Now we're going to add a pre-build step to our Rust project. Create a new file (in the same directory as `Cargo.toml`), named `build.rs`:

```rust
fn main() {
    cc::Build::new()
        .file("src/mylib.c")
        .compile("mylib");
}
```

When you run `cargo build` (or run, etc.), `build.rs` is executed first. It calls the `cc` helper to compile your C for you, and link it into your binary.

Now let's call the C. In `src/main.rs`:

```rust
extern {
    fn double_it(x: i32) -> i32;
}

fn main() {
    let i = 42;
    let j = unsafe { double_it(i) };
    println!("{} * 2 = {}", i, j);
}

```

Notice that the C call is marked `unsafe`. That doesn't mean its bad, it means that it can't be verified by Rust.