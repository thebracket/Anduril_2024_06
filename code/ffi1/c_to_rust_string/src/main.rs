mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    // The "c" prefix is a helper for making static C strings!
    let my_c_string = c"Hello World";

    unsafe {
        mylib_c::print_message(my_c_string.as_ptr());
    }

    // Or you can go from a regular string
    let my_string = "Hello String";
    let my_c_string = std::ffi::CString::new(my_string).unwrap();
    unsafe {
        mylib_c::print_message(my_c_string.as_ptr());
    }
}
