# Borrowing

Moving everything into functionas and moving them back again is often pretty unwieldy. What you want is a *reference*---just like C++!

```rust
fn my_function(s: &String) {
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    my_function(&s);
    println!("{s}");
}
```

So the semantics are basically the same as C++ here: you denote `&String` as a reference to a string.