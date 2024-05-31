# Type Conversions

See: [https://cxx.rs/bindings.html](https://cxx.rs/bindings.html)

Many Rust types are implemented in C++ via the `cxx` library. For example:

* `String` in Rust is `rust::String` in C++.
* `Vec<T>` in Rust is `rust::Vec<T>` in C++.
* `Box<T>` in Rust is `rust::Box<T>` in C++.

Some types are mapped the other way around, for example:

* `CxxVector<T>` in Rust is `std::vector` in C++.
* `CxxString` in Rust is `std::string` in C++.

Some such as `unique_ptr`, `shared_ptr` are mapped directly.

You can use pointers between the two languages, including function pointers.

> It's not a complete implementation, but it's often enought to let you minimize the changes required to start using existing C++ from Rust.