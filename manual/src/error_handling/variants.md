# Walkthrough: Different Error Types

So let's say that you like your program and don't want to crash by unwrapping everything error related. You'd also like to not write a `match` or `if let` statement for every single error, and you'd like to use the ergonomic `?` operator.

The problem is that Rust errors are *really* specific.

Let's start with a simple function:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("mytile.txt");
    std::fs::read_to_string(my_file)
}

fn main() {
    match maybe_read_a_file() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

So far so good. `read_to_string` returns an `std::io::Error` - so we'll just return that, right? We could add another function to make it uppercase:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("mytile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

fn main() {
    match file_to_uppercase() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

So far so good, and you can see the `?` worked nicely because we're still sending the same result type. But what if we also want to parse the file from JSON?

```rust
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

fn load_users() -> Result<Vec<User>, ???> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
```

Here we have a problem. `read_to_string` gives an IO error. Parsing Json gives a Serde error. They aren't the same type, and you have to have a single type returned from your function! You *could* define your own error type and use `map_err` to transform everything into your own error type (we'll talk about `thiserror` in a moment).

Think back to the previous section. We can use a:

```rust
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users() -> Result<Vec<User>, GenericResult> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
```

Because it's a boxed dynamic type, it'll accept whatever errors you put in there.