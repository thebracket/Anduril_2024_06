# Walkthrough: instantiate a C++ class and use it

Create a new Rust project and add `cxx` as a dependency:

```toml
[package]
name = "simple_class"
version = "0.1.0"
edition = "2021"

[dependencies]
cxx = "1.0"

[build-dependencies]
cxx-build = "1.0"
```

Now let's build a header file. In `include/simple_class.h`:

```h
#pragma once
#include <memory>

class SimpleClass {
    public:
    SimpleClass();
    void say_hello() const;
    ~SimpleClass();

    // An example of mutable class methods, which are a little harder.
    void set_counter(uint64_t value);

    private:
    uint64_t counter;
};

std::unique_ptr<SimpleClass> create_simple_class();
```

And in `src/simple_class.cpp`:

```cpp
#include "simple_class.h"
#include <iostream>

SimpleClass::SimpleClass() {
    std::cout << "SimpleClass constructor\n";
    this->counter = 1;
}

SimpleClass::~SimpleClass() {
    std::cout << "SimpleClass destructor\n";
}

void SimpleClass::set_counter(uint64_t value) {
    this->counter = value;
}

void SimpleClass::say_hello() const {
    for (int i = 0; i < this->counter; i++) {
        std::cout << "Hello from SimpleClass run (" << i << ")\n";
    }
}

std::unique_ptr<SimpleClass> create_simple_class() {
    return std::make_unique<SimpleClass>();
}
```

So we've built a simple class that tells you when it is constructed or destructed. We've also exposed a function that creates an instance of the class in a `unique_ptr`.

Let's write out `main.rs` to use it:

```rust
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // List each header to include
        include!("simple_class.h");

        // List classes as namespaces
        type SimpleClass;

        // Const methods are easiest, you can just use &self
        fn say_hello(&self);

        // Mutable methods require the Pin system below for `self`
        fn set_counter(self: Pin<&mut SimpleClass>, counter: u64);

        // Free function that creates an instance of the class.
        fn create_simple_class() -> UniquePtr<SimpleClass>;
    }

    extern "Rust" {
        // This is where we'll put functions to go the other way
    }
}

fn main() {
    // We create a unique ptr class. You'll see the constructor run.
    let mut simple_class = ffi::create_simple_class();

    // Calling say_hello is easy - it's immutable
    simple_class.say_hello();

    // You have to "pin" mutable methods to stop memory rearranging.
    simple_class.pin_mut().set_counter(2);    
    simple_class.say_hello();

    // The destructor fires
}
```

There's two halves here:

* The `cxx:bridge` triggers the CXX library to generate intermediary types. We include the header file, define a Rust type alias for `SimpleClass` and create a function signature.
* In `main()`, we actually run the program.

We still have to actually compile the thing. So we need a `build.rs` file:

```rust
// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/simple_class.cpp")
        .include("include")
        .std("c++14")
        .compile("simple_class");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/simple_class.cpp");
    println!("cargo:rerun-if-changed=src/simple_class.h");
}
```

Running the program with `cargo run` compiles both the Rust and the C++, joins them together, and prints:

```
SimpleClass constructor
Hello from SimpleClass << (0)
Hello from SimpleClass << (0)
Hello from SimpleClass << (1)
SimpleClass destructor
```

It works!