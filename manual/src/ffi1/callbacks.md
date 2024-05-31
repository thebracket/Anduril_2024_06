# Callbacks and Function Pointers

Let's get ambitious and call into C, and give it a Rust function as a callback!

Adjust your `mylib.h`:

```h
void callme(void (*callback)(int));
```

And the `mylib.c`:

```c
void callme(void (*callback)(int)) {
    printf("Calling a Rust function from C\n");
    callback(42);
}
```

Once again, let `bindgen` do its job with a `cargo build`.

Let's try and make a callback work in `main.rs`:

```rust
mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

unsafe extern "C" fn callback(n: i32) {
    println!("Callback called from C code");
    println!("n = {}", n);
}

fn main() {
    unsafe {
        mylib_c::callme(Some(callback));
    }
}
```

So this is slightly different, but highlights what you need to do to go the other way:

* Functions called from C are `unsafe`.
* To enable the C calling convention, you have to flag the external function `extern "C"`
* Function pointers can be `null` in C land, so they are converted to an `Option`.