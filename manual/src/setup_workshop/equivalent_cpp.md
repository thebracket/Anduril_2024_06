# Equivalent C++

> C++ source code for this project is in `cpp/hello_world`.

A simple C++ equivalent program is as follows:

```c++
#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
```

> Not everyone likes `iostream`. If you prefer `printf` or any of the other output systems, that's cool too.

It is accompanied by a CMakeLists.txt file:

```haskell
cmake_minimum_required(VERSION 3.5)
project(HelloWorld)

add_executable(hello hello.cpp)
```

And you can build the project and execute it with the following:

```bash
# First time only
mkdir build
cd build
cmake ..

# And then
cd build
make
./hello
```

This will give you the expected output: `Hello, World!`.

### Comparing Cargo.toml and CMake

The `Cargo.toml` and `CMakeLists.txt` files are similar: you specify the project details, and `CMake` builds the builder for you. Rust is doing a few more things:

* Your executable is statically linked, and includes the Rust standard library (that's why the executable is so much larger).
* `Cargo` includes dependency management, so your CMake file really includes `vcpkg`, `Conan` or one of the other build tools.
* `Cargo` doesn't offer to create makefiles, Ninja build systems, etc. --- it's an all in one tool.

So in reality, your `CMakeLists.txt` file would be a bit bigger:

```haskell
cmake_minimum_required(VERSION 3.5)
project(HelloWorld)

# Example vcpkg (commented out because I don't have it installed)
#set(CMAKE_TOOLCHAIN_FILE ~/vcpkg/scripts/buildsystems/vcpkg.cmake CACHE FILEPATH "Path to toolchain")

# Example static linking
#set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")

add_executable(hello hello.cpp)
```

### Comparing main.rs with hello.cpp

The files are quite similar. 

1. C++ brings in `iostream` with `#include <iostream>`. You don't need to do that for `println!`---Rust makes it available by default.
2. Both define a main function. `fn main()` and `int main()` are almost equivalent---but the Rust version doesn't return anything.
3. `println!` and `std::cout << "Hello, World!" << std::endl;` are equivalent. `println!` adds a `\n` to the end for you, which triggers a flush (`println!` is unbuffered). If you want to not emit a `\n`, you can use `print!`.
4. `return 0` returns an exit code of 0. Rust programs do that for you by default.

So despite the syntax being different, it's not all that different.

If you *really* want to be returning an exit code, you can use the following:

```rust
use std::process:ExitCode;

fn main() -> ExitCode {
    println!("Hello, world!");
    return ExitCode::from(0);
}
```

And that concludes our quick "Hello World" tour. We've covered:

* How to create a Rust program with Cargo.
* What the resulting files and structure cover.
* An equivalent C++ and CMake setup.
* Exit codes.
