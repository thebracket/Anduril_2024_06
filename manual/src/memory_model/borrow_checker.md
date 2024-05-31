# The Borrow Checker

> *Show of Hands*. `mut` means *mutable*. Does anyone know what else it means?

We're going to get to that. But `mut` *also* means a compile-time "mutual exclusion" wrapper is applied. A variable can only be borrowed mutably *once*, exclusive-or any number of immutable borrows. You can immutably borrow as much as you want---it's just a read. But you can't immutably borrow at the same time as you mutably borrow, nor can you have more than one mutable borrowers at a time.

That's the foundation of the borrow checker. We'll go into what it means in a bit.

When you first hit the borrow checker, it's usually because you tried something like this:

```rust
fn main() {
    let mut array = [1, 2, 3,4 ,5];
    for (i, entry) in array.iter().enumerate() {
        if i > 0 {
            array[i - 1] = *entry;
        }
    }
}
```

This results in the error message:

```
error[E0506]: cannot assign to `array[_]` because it is borrowed
```

**So why did that fail?**

* Calling `iter` has created an iterator that traverses the array.
* Iter operates on references (you can use `into_iter()` to move everything out of the array instead if you want to).
* Since `iter` has an immutable borrow, the mutual exclusion rule kicks in: you can't mutably borrow something that is already borrowed (mutably or immutably).

> It's even a good thing. If you allow mutation mid iteration, you are risking *iterator invalidation*---a remarkably common bug. In this case, it'd be harmless. In a lot of cases, it could be catastrophic!

In this case, you *can* rewrite it using index access and it works fine:

```rust
fn main() {
    let mut array = [1, 2, 3,4 ,5];
    for i in 1 .. array.len() {
        array[i-1] = array[i];
    }
}
```

Let's take a break. After the break, we'll workshop some threads---and start to see why the memory model is on your side.