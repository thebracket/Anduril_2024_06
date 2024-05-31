extern {
    fn double_it(x: i32) -> i32;
}

fn main() {
    let i = 42;
    let j = unsafe { double_it(i) };
    println!("{} * 2 = {}", i, j);
}
