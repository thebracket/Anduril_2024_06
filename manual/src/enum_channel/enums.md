# Rust Enumerations

You're probably used to thinking of enumerations as a nicely named integer:

```cpp
enum Foo { a, b, c = 10, d, e = 1, f, g = f + c };
```

This is super-useful, and Rust can do it too:

```rust
fn main() {
    enum Foo {
        A,
        B,
        C = 3,
    }

    let my_foo = Foo::C;
    let my_int: i32 = my_foo as i32;
    println!("{my_int}");
}
```

You can even specify the underlying type (by default it'll be the smallest unsigned integer into which the enumeration options fit):

```rust
#[repr(u8)]
enum Foo {
    A,
    B,
    C = 3,
}
```