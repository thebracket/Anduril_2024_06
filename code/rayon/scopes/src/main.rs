fn main() {
    let i = 5;
    rayon::scope(|scope| {
        scope.spawn(|_scope| println!("{i}"));
    });
}