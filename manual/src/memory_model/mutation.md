# Mutation

So how about if you want to actually *change* variables? You might not like the functional style of always defining new variables on every change. Even Haskell lets you sneak in Monad functions when you want mutation...

This won't work:

```rust
fn main() {
    let i = 1;
    i += 2;
    println!("{i}");
}
```

Rust correctly errors out telling you that you can't assign twice to an immutable variable--and everything is `const` by default in Rust (except where it isn't... we'll go there).

So the `mut` keyword lets you have a good old mutable variable:

```rust
fn main() {
    let mut i = 1;
    i += 2;
    println!("{i}");
}
```

(The answer is 3)
