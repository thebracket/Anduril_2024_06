fn main() {
    cc::Build::new()
        .file("src/mylib.c")
        .compile("mylib");
}