# Pattern Matching

Rust's `match` statement is a bit like `switch` on steroids. As well as working like a regular switch, it can do full pattern matching.

```rust
enum Thing {
    Vehicle { num_wheels: usize },
    Table { capacity: usize },
    Person { name: String, height: usize },
    TupleType(usize),
}

fn main() {
    // A simple switch-like match (no fall through)
    let i = 5;
    match i {
        5 => println!("You got it!"),
        _ => println!("Nope!"), // _ is the "match everything" option
    }

    // Match with field extraction
    let thing = Thing::Table { capacity: 12 };
    match thing {
        Thing::Vehicle{..} => {} // We're doing nothing for vehicles. The .. ignores all fields.
        Thing::Table { capacity } => {
            // We've destructured "table" into a local variable in this block
            println!("Table for {capacity}");
        }
        Thing::Person { name, .. } => {
            // We've destructured 'name' and opted to ignore the other fields
            println!("A person named {name}");
        }
        Thing::TupleType(n) => {
            // You can extract from tuples in the same way. It's just a
            // struct without field names.
            println!("{n}");
        }
    }
}
```

If you only care about a single possible answer, `if let` is a "single option match":

```rust
enum Thing {
    Vehicle { num_wheels: usize },
    Table { capacity: usize },
    Person { name: String, height: usize },
    TupleType(usize),
}

fn main() {
    let thing = Thing::Table { capacity: 12 };
    if let Thing::Table { capacity } = thing {
        println!("Table for {capacity}");
    } else {
        println!("It didn't match");
    }
}
```

In turn, `if let` gives you a nice way of handling errors or options (Option replaces `null` to *force* you to at least consider the null case). Instead of unwrapping:

```rust
fn main() {
    let i = 12i32;
    if let Some(n) = i.checked_div(0) {
        println!("{n}");
    } else {
        println!("Division by zero is bad for your health");
    }
}
```