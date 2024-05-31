# Task: Make an Animal Trait

Go ahead and (in a new project):

* Define a trait named `Animal`.
* The trait requires a function called `make_noise`.
* Create a `Dog` and a `Cat` type, each of which implements `make_noise` and emits with `Woof` or `Meow`.
* In `main`, create a cat and a dog variable and have them each make a noise.

This is similar to a lot of OOP tasks. Very simple.

![](../images/ScrollTime.png)

Here's my version:

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
    let cat = Cat;
    let dog = Dog;
    cat.make_noise();
    dog.make_noise();
}
```