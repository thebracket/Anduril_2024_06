# Passing Parameters 2

Therefore, you'd reasonably expect this to work, too:

```rust
fn my_function(s: String) {
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    my_function(s);
    println!("{s}");
}
```

> Spoiler: It won't compile with a huge error message about "borrow of moved value"
