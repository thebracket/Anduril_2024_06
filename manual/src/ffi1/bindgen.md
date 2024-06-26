# Bindgen for Automatic Headers

Writing all your own `extern` blocks exactly mirroring all the C isn't fun! Mozilla created a tool called `bindgen` to help with this.

Let's start by adding a header file to `mylib`. Create `mylib.h`:

```h
int double_it(int x);
```

And add the include to your `mylib.c` file for completeness:

```c
#include "mylib.h"

// A really simple function that doubles a number
int double_it(int x) {
    return x * 2;
}
```

Now we're going to add a second build dependency:

```toml
[build-dependencies]
cc = "1"
bindgen = "0"
```

And adjust out `build.rs` file to run `bindgen` to generate the code for us from the header file:

```rust
use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/mylib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build and link the C
    cc::Build::new().file("src/mylib.c").compile("mylib");
}
```

When you run `cargo build`, your headers are parsed and placed in a file named `bindings.rs` inside your target build directory. Here's what it came up with for me:

```rust
/* automatically generated by rust-bindgen 0.69.4 */

extern "C" {
    pub fn double_it(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
```

Now you can replace your `extern` block with a macro to import the created headers:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let i = 42;
    let j = unsafe { double_it(i) };
    println!("{} * 2 = {}", i, j);
}
```

## Some Best Practices

### Use a Module

It's common to wrap the `bindings.rs` import in a module:

```rust
mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
```

### Provide Wrappers

If you trust the code:

```rust
mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub double_it(n: i32) {
    unsafe {
        mylib_c::double_it(n)
    }
}
```

## Unit Test as you Port

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_it() {
        for i in 0..20 {
            assert_eq!(double_it(i), unsafe { mylib_c::double_it(i) });
        }
    }
}
```