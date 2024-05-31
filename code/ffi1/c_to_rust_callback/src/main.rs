mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

unsafe extern "C" fn callback(n: i32) {
    println!("Callback called from C code");
    println!("n = {}", n);
}

fn main() {
    unsafe {
        mylib_c::callme(Some(callback));
    }
}
