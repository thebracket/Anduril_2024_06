# Step-by-Step Spinlock

This is based on Mara Bos' example from her excellent book *Rust Atomics and Locks*. You can read the full version for free at [https://marabos.nl/atomics/building-spinlock.html](https://marabos.nl/atomics/building-spinlock.html).

A `Spinlock` is a type of lock that repeatedly tries to acquire access to the lock, "spinning" in a busy loop until it succeeds. Under high contention, this can lead to high CPU usage (but fast lock acquisition when available). In low contention, it is practically instant.

Spinlocks are used a lot in the Linux kernel for possibly contended devices/resources. In many cases, a well-crafted spinlock is quite a bit faster than a `Mutex`---if you don't mind the potential for higher CPU usage.