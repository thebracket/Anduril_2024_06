fn main() {
    rayon::join(|| println!("Hello"), || println!("World"));
}
