# Code Along: Cats and Dogs, Living Together

If you want to put cats and dogs together into a vector, you can't do the obvious:

```rust
let madhouse = vec![Cat, Dog];
```

You instead have to use:

```rust
trait Animal {
    fn make_noise(&self);
}

struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow");
    }
}

struct Dog;

impl Animal for Dog {
    fn make_noise(&self) {
        println!("Woof");
    }
}

fn main() {
    let madhouse: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat), Box::new(Dog)
    ];

    for a in madhouse.iter() {
        a.make_noise();
    }
}
```

## What's Going on Here?

* The `dyn` keyword means "dynamic". It typically means a vtable will be involved.
* Rust can't determine what size an `Animal` is (in bytes), and vectors have to be of a uniform size. So you put each `Animal` in a box. So you have a vector of pointers to dynamic objects.

You have exactly the same thing as `std::vector<unique_ptr<Animal>>`!