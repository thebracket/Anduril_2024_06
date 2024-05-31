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