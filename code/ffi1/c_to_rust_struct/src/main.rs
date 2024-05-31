mod mylib_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let mut data = mylib_c::MyStruct { x: 1, y: 2 };

    unsafe {
        mylib_c::print_struct(data);
        mylib_c::print_ptr_to_struct(&mut data as *mut mylib_c::MyStruct);
    }
}
