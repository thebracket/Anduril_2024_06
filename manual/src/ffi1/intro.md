# FFI Part 1

We're going to start with the easiest type of FFI (Foreign Function Interface) - interfacing with C (or C++ representing C). Rust is really good at this, and an explicit design decision early on was that there shouldn't be any marshalling penalty when calling between the languages.
