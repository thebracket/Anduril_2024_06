# Error Handling

Over the years, error handling has evolved:

Language|Error Handling
--|--
C|Return an error number
C++|Throw an exception
C++ in `noexcept`|Return an error number
Modern C++|Exception or an `std::expected`
Go|Return BOTH a success variable and an error and make you check the error
Rust|Return an `enum` sum type that is an error or the result.

There's much to be said for all of them, but Rust does not have exceptions.