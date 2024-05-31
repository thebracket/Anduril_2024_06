#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("simple_class.h");

        type SimpleClass;

        fn say_hello(&self);

        // This is particularly nasty, sorry.
        fn set_counter(self: Pin<&mut SimpleClass>, counter: u64);

        fn create_simple_class() -> UniquePtr<SimpleClass>;
    }

    extern "Rust" {

    }
}

fn main() {
    let mut simple_class = ffi::create_simple_class();
    simple_class.say_hello();
    simple_class.pin_mut().set_counter(2);
    simple_class.say_hello();
}
