# Passing Parameters

You'd reasonably expect this to work:

```rust
fn my_function(a: i32) {
    println!("a equals {a}");
}

fn main() {
    let my_var = 12;
    my_function(my_var);
    println!("{my_var}");
}
```

Run the demo.

> Spoiler: it does work.