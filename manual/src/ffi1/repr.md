# Representations

Rust can (and will) rearrange your structures. That's often good for performance, but it's a really horrible thing to do to you if you are trying to share a pointer to a structure with C - and the two languages have to agree on layout!

You can tell Rust that it should use a C-compatible representation for a structure and not rearrange things:

```rust
#[repr(C)]
struct Example {
    a: i32,
    b: i64,
}
```

You pretty much *have* to do this for structures you are sharing with C. Otherwise, you will be chasing some very strange bugs.