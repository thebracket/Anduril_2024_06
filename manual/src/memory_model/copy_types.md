# Copy Types

## So why did the first one work?

Some types---primitives that fit in a register---are `copy types`. They have the `Copy` trait implemented automatically, and won't be cloned or moved when used as a parameter --- they are just copied. This allows the compiler to just put them in a register and skip potentially slow operations.

## So why did the second one NOT work?

Most types are *not* `Copy`. Types aren't `Copy` by default---they have to add:

```rust
#[derive(Copy, Clone)]
```

To their declaration. You *can't* declare `Copy` on types that contain anything that isn't already `Copy`---and on types that aren't simple register copies.

Rust is *move by default*.