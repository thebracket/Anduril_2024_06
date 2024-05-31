# Enumerations as Sum Types

Rust enumerations can *also* work a lot like `std::variant` or a C tagged union type. Enumeration entries can contain data (and will be the size of the largest option).

For example:

```rust
enum Things {
    Vehicle { num_wheels: usize },
    Table { capacity: usize },
    Person { name: String, height: usize },
    TupleType(usize),
}
```

You can even associate functions with enumerations:

```rust
enum Animal {
    Dog,
    Cat,
}

impl Animal {
    fn noise(&self) {
        match self {
            Animal::Dog => println!("Woof"),
            Animal::Cat => println!("Meow"),
        }
    }
}

fn main() {
    let animals = [ Animal::Cat, Animal::Dog ];
    animals
        .iter()
        .for_each(|a| a.noise());
}
```

Enumerations can also be generic:

```rust
#[derive(Debug)]
enum MyOption<T: std::fmt::Debug>  {
    None,
    Some(T),
}

fn main() {
    let a: MyOption<String> = MyOption::None;
    let b = MyOption::Some("Hello".to_string());

    println!("{a:?}");
    println!("{b:?}");
}
```

If that looks familiar, it's because `Result` and `Option` are both enumerations. Just like ones you can create yourself (but with lots of functionality baked in).

Just like a `variant`, options are *sum types*. Only one option is enabled and you can't treat it as if it were another option.