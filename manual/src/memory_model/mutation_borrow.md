# Mutation and Borrowing

Rust is explicit about this, too. If you want a function to be able to *change* a borrowed variable:

* The variable has to be mutable (`mut`).
* The function signature reference has to be marked as mutable, too.
* The caller signature reference has to also be marked as mutable.

That's a lot of `mut`.

```rust
fn add_one(n: &mut i32) {
    *n += 1; // Note the dereference to access the actual primitive
}

fn main() {
    let mut i = 0;
    add_one(&mut i);
    println!("{i}");
}
```

So once again, this is protecting you from the function signature on a reference suddenly deciding that it isn't constant anymore, and side-effects flowing through your program. It's extra typing again, but you're *making your intent clear*. If the function signature changed, the caller has to change, too.

