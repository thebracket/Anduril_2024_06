# Defining a Trait

Defining a trait is easy:

```rust
trait MyTrait {
    fn do_something(&self);

    fn optional(&self) {
        println!("The trait implemented a default for me");
    }
}

struct MyType;

impl MyTrait for MyType {
    fn do_something(&self) {
        println!("I did it!");
    }
}

fn main() {
    let t = MyType;
    t.do_something();
    t.optional();
}
```