# Linking C into Rust - sys crates

Just a quick note. You'll find lots of crates named `*_sys`. If you are making a crate that wraps some C, it's convention to name it `_sys`. It's quite common to then make a crate with an actual name that ports the code, or just offers a Rust-friendly wrapper.

Rust and C play nicely together. There are *thousands* of these.