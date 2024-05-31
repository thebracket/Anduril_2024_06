# Move by Default

So this works:

```rust
fn my_function(s: String) -> String {
    println!("{s}");
    s
}

fn main() {
    let s = "Hello".to_string();
    let s = my_function(s);
    println!("{s}");
}
```

Kinda unwieldy, but not as inefficient as it looks because Rust has RVO (Return Value Optimization) straight out of the C++ portions of LLVM. You probably don't always want to be doing that---but it works.
