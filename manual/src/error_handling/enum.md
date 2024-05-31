# Result is an emum

A Result is a generic enumeration:

```rust
enum Result<T, E: Error> {
    Ok(T),
    Error(E),
}
```

The `Error` type must implement the `Error` trait. Fortunately, `String` does - so if you're lazy (or just like allocation) you can just use strings everywhere... the standard library doesn't do that, and I wouldn't recommend it either.

`Result` does support a number of ways to handle an error:

* `unwrap` - crash if it's an error, return the contents if its not.
* `expect` - crash, but with a nice error message.
* `or` - substitute a default for an error.
* `or_else` - substitute the return of a different function call if an error
* `map_err` - turn the error into something else entirely
* (and many more including `and_then`...)

And just like any other enum, you can match on it:

```rust
if let Ok(v) = try_something() {
    ...
} else {
    ...
}
```

There's also the helpful `?` operator, which either transforms a value into the `Ok()` version, or propagates the error out of the function - which starts to sound a lot like exceptions!

