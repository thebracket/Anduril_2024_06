# Task: Boxed Dynamic Errors

Make a very quick project:

* Use `read_to_string` to load a text file.
* Use `parse` to turn it into an integer.
* Use both in the same function and the `?` operator.
* Make `main` return an error, too.

![](../images/ScrollTime.png)

Here's my version:

```rust
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_and_parse() -> GenericResult<i32> {
    let file = std::fs::read_to_string("myfile.txt")?;
    let parsed: i32 = file.parse()?;
    Ok(parsed)
}

fn main() -> GenericResult<()> {
    load_and_parse()?;
    Ok(())
}
```