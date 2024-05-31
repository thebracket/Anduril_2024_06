mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn double_it(x: i32) -> i32 {
    unsafe { mylib_c::double_it(x) }
}

fn main() {
    let i = 42;
    let j = unsafe { mylib_c::double_it(i) };
    println!("{} * 2 = {}", i, j);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_it() {
        for i in 0..20 {
            assert_eq!(double_it(i), unsafe { mylib_c::double_it(i) });
        }
    }
}