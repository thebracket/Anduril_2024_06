# Advice: thiserror and anyhow

A *lot* of projects use a crate named `anyhow` while developing---and a lot of applications use it in production.

`anyhow::Result` is an extended version of the generic result with added options for displaying context, downcasting to specific error types, etc.

You'll find a lot of this:

```rust
fn main() -> anyhow::Result<()> {
    // Lots of calls with ? at the end
    Ok(())
}
```

`thiserror` is a helper crate to define your own errors:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers, 
    #[error("Too many users were found")]
    TooManyUsers
}
```

It's good practice if you are writing a library for others to use to handle errors gracefully and return your own error types.