# Generics and Traits

You can pass objects around by trait, making functions require them:

```rust
trait MyTrait {
    fn do_it(&self) {
        println!("I did it!");
     }
}

fn print(t: impl MyTrait) {
    t.do_it()
}

struct MyType {}
impl MyTrait for MyType {}

struct MyType2 {}
impl MyTrait for MyType2{}

fn main() {
    let t = MyType{};
    let t2 = MyType2{};
    print(t);
    print(t2);
}
```

> `impl Trait` is a shorthand added in the last couple of years.

If you need a bit more complexity, you can declare the function generic:

```rust
trait MyTrait {
    fn do_it(&self) {
        println!("I did it!");
     }
}

fn print<T: MyTrait>(t: T) {
    t.do_it()
}

struct MyType {}
impl MyTrait for MyType {}

struct MyType2 {}
impl MyTrait for MyType2{}

fn main() {
    let t = MyType{};
    let t2 = MyType2{};
    print(t);
    print(t2);
}
```

> This is *exactly* the same as using a C++ template in this case. One function instance will be created in the binary per type passed in.

You can make the function require multiple traits:

```rust
trait MyTrait {
    fn do_it(&self) {
        println!("I did it!");
     }
}

fn print<T: MyTrait+Clone>(t: T) {
    t.do_it()
}

#[derive(Clone)]
struct MyType {}
impl MyTrait for MyType {}

struct MyType2 {}
impl MyTrait for MyType2{}

fn main() {
    let t = MyType{};
    //let t2 = MyType2{};
    print(t);
    //print(t2); // t2 will not compile!
}
```

You can make the trait itself require other traits:

```rust
trait MyTrait: Clone {
    fn do_it(&self) {
        println!("I did it!");
     }
}

fn print<T: MyTrait>(t: T) {
    t.do_it()
}

#[derive(Clone)]
struct MyType {}
impl MyTrait for MyType {}

struct MyType2 {}
// MyType2 won't compile because it doesn't meet the trait requirements
//impl MyTrait for MyType2{}

fn main() {
    let t = MyType{};
    //let t2 = MyType2{};
    print(t);
    //print(t2); // t2 will not compile!
}
```