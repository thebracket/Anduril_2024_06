# The Any Trait

How many people rely on downcasting in C++? Hopefully not everyone. It's a little fraught, sometimes!

Rust has this available via the `Any` trait:

```rust
use std::any::Any;

trait Animal: Any {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
    
    fn as_any(&self) -> &dyn Any;
}

struct Cat;
struct Tortoise;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat), Box::new(Tortoise)
    ];

    for animal in animals.iter() {
        if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
            println!("We have access to the cat");
        }
        animal.make_noise();
    }
}
```