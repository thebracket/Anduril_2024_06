# Task: Make a generic function and structure

Keeping your existing `Cat` and `Dog`:

* Add a field named `age` to your `Cat` and `Dog` types.
* Create a second trait named `Age`.
* Add a method `get_age`, and implement it in each type to return the age.
* Create a generic function that requires its input be both an `Animal` and have an `Age`.
* Call the function to print the creatures ages.

![](../images/ScrollTime.png)

Here's my version:

```rust
trait Animal {
    fn make_noise(&self);
}

trait Age {
    fn get_age(&self) -> i32;
}

struct Cat { age: i32 }

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow");
    }
}

impl Age for Cat {
    fn get_age(&self) -> i32 { self.age }
}

struct Dog { age: i32 }

impl Animal for Dog {
    fn make_noise(&self) {
        println!("Woof");
    }
}

impl Age for Dog {
    fn get_age(&self) -> i32 { self.age }
}

fn show_age<T: Animal+Age>(t: T) {
    println!("Age: {}", t.get_age());
}

fn main() {
    let cat = Cat{age: 12};
    let dog = Dog{age: 3};
    show_age(cat);
    show_age(dog);
}
```

## Items That Often Come Up

* Traits really can't have their own data members. That surprises everyone coming from C++!
