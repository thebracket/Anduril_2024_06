# Interior Mutability and Safety

We've mentioned "Interior Mutabilty" a few times, and promised to go over it. It's a core pattern to more advanced Rust, but can be a little counterintuitive at first.

There were a couple of items we noted earlier:

* Rust's `Mutex` and other synchronization structures "wrap" their contents - you can't get to the inner data without locking the synchronization primitive.
* The `Mutex` itself doesn't have to be mutable, even though you can mutate its contents.

In this section, we're going to work through why that's the case - and how it helps you.