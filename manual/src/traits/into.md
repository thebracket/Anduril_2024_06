# The Magic of Into

You can convert from one type to another with `into()` instead of `as i32`. It's a good habit to get into: Rust only implements `into()` for safe conversions.

This compiles:

```rust
fn main() {
    let i: i32 = 12;
    let j: i64 = 12.into();    
}
```

You don't want the reverse to compile:

```rust
fn main() {
    let i: i64 = i32::MAX as i64 + 1;
    let j: i32 = i as i32;
    println!("{j}");
}
```

(Doesn't compile)

```rust
fn main() {
    let i: i64 = i32::MAX as i64 + 1;
    let j: i32 = i.into();
}
```

## Implementing Your Own Conversions

```rust
struct Meter(f32);
struct Centimeter(f32);

impl From<Meter> for Centimeter {
    fn from(m: Meter) -> Centimeter {
        Centimeter(m.0 / 10.0)
    }
}

fn main() {
    let distance = Meter(1.0);
    let cm: Centimeter = distance.into();
    println!("{} cm", cm.0);
}
```

Setup enough of these, and you can start making units of measure generic:

```rust
struct Meter(f32);
struct Centimeter(f32);

impl From<Meter> for Centimeter {
    fn from(m: Meter) -> Centimeter {
        Centimeter(m.0 / 10.0)
    }
}

impl From<Centimeter> for Meter {
    fn from(m: Centimeter) -> Meter {
        Meter(m.0 * 10.0)
    }
}

fn measure<D: Into<Meter>>(distance: D) {
    println!("Distance in Meters: {}", distance.into().0)
}

fn main() {
    measure(Meter(1.0));
    measure(Centimeter(1.0));
}
```

Using "strong types" ("New Types" in Rust parlance) is a best practice in C++. It's also a best practice in Rust. Let the type system help you!