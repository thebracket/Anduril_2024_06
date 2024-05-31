# Walkthrough: adding Rust to C++

Adding Rust to your C++ can also be done through `cxx.rs`.

In your `main.rs` file, add:

```rust
extern "Rust" {
    pub fn callback();
}
```

> You can expose types in the same way. Declare `type MyType;` in the extern block.

Then below `main`, add the implementation:

```rust
pub fn callback() {
    println!("Callback called!");
}
```

In the `simple_class.cpp` file:

```cpp
// This is auto-generated on build
#include "simple_callback/src/main.rs.h"

SimpleClass::SimpleClass() {
    std::cout << "SimpleClass constructor\n";
    this->counter = 1;
    callback();
}
```

And run the program and you get:

```
SimpleClass constructor
Callback called!
Hello from SimpleClass run (0)
Hello from SimpleClass run (0)
Hello from SimpleClass run (1)
SimpleClass destructor
```
