# Linking Rust into C

Sometimes you want to go the other way: write some Rust, and use it as a C library. We won't go into that in as much detail (most of it is very similar), but it's also well supported and widely used.

## Cargo.toml

Create a new library project (`cargo new rust_to_c --lib`). Edit `Cargo.toml`:

```toml
[package]
name = "rust_to_c"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
libc = "0.2"

[build-dependencies]
cbindgen = "0.24"
```

And change `lib.rs` to:

```rust
use std::ffi::CStr;

/// # Safety
/// Use a valid C-String!
#[no_mangle]
pub unsafe extern "C" fn hello(name: *const libc::c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    println!("Hello {name}");
}
```

We're using the unsafe, C calling convention again. We also have to make sure our parameters are C-friendly.

Now we'll build a `build.rs` to make a header file for us:

```rust
use std::env;
use std::path::PathBuf;
use cbindgen::Config;


fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let config = Config {
        //namespace: Some(String::from("ffi")),
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file(&output_file);
}

/// Find the location of the `target/` directory. Note that this may be 
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR` 
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
```

Now run `cargo build`.

In `target/rust_to_c.hpp` it has created:

```h
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// # Safety
/// Use a valid C-String!
void hello(const char *name);

} // extern "C"
```

It's not perfect and uses too many headers, but there it is.

It's also created `librust_to_c.so` (on Linux; filenames vary by target).

You can now link to and use the library as you would any other C library.