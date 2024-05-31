# Naked Pointers

> The *vast* majority of Rust code uses smart pointers, and when possible - so should you. Just like Modern C++, safety comes from having automated mechanisms to not leak or overflow memory!

This is particularly important when doing interop with C code.

Naked pointers in Rust look a bit like C, but with an added mutability tag:

```rust
raw_memory: *const u8
mutable_raw_memory: *mut u8
```

You can do pointer arithmetic:

```rust
fn main() {
    let data = [1, 2, 3, 4, 5];
    let ptr: *const i32 = data.as_ptr();
    println!("{}", unsafe { *ptr });
    println!("{}", unsafe { *ptr.offset(1) });
}
```

> Notice that dereferencing is `unsafe`. You're running into the aliasing rules by having a pointer to something already represented in memory.

You can even do really unfortunate pointer arithmetic that sucks in C++ too:

```rust
fn main() {
    let data = [1, 2, 3, 4, 5];
    let ptr: *const i32 = data.as_ptr();
    println!("{}", unsafe { *ptr.offset(5) });
}
```

This printed `836047640` on the Rust playground! Rust isn't providing a managed runtime - it really is a systems language. So once you take off the seatbelts, you can have exactly the same bugs as C and C++!

You can mutate data with pointers:

```rust
fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let ptr: *mut i32 = data.as_mut_ptr();
    unsafe { *ptr = 42; }
    println!("{data:?}");
}
```

You can even directly allocate memory and use it:

```rust
use std::alloc::Layout;
use std::alloc::{alloc, dealloc};

fn main() {
    let (layout, mut ptr) = unsafe {
        let layout = Layout::array::<u8>(128).unwrap();
        let ptr = alloc(layout);
        (layout, ptr)
    };

    for i in 0..128 {
        unsafe { *(ptr.offset(i)) = i as u8; }
    }

    for i in 0..128 {
        unsafe { println!("{}", *ptr.offset(i)); }
    }

    unsafe {
        dealloc(ptr, layout);
    }
}
```

> In other words, most of what you know about pointers from C and C++ is still true. You just have to be careful to differentiate between `* const` and `* mut`. And, of course, if you're mucking with memory directly---you need `unsafe` and to be careful.

You can use Rust's [std::mem::transmute](https://doc.rust-lang.org/std/mem/fn.transmute.html) to translate bytes into types directly (a direct cast). Be careful, you can very easily make unsound code this way. There's usually helper functions to avoid the need for unsafe code:

```rust
fn main() {
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = unsafe {
        std::mem::transmute::<[u8; 4], u32>(raw_bytes)
    };

    // use `u32::from_ne_bytes` instead
    let num = u32::from_ne_bytes(raw_bytes);
    // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);
}
```

Finally, the `zerocopy` and `ByteMuck` crates provide a safe interface for much of this.

