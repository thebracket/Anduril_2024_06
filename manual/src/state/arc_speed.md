# Arc Overhead

The speed of cloning an `Arc` is equal to the cost of an atomic fetch/add on your platform. With little contention (you're not cloning it continually!), on an Intel i7 that's around 15-20 nanoseconds. It "skyrockets" to nearly 50 nanoseconds with 5 threads contending. (Source: data sheets).

In other words, unless you are doing something with nanosecond resolution---the overhead of sharing resources and state via an `Arc` is negligible.

These numbers are suspiciously close to a L2 cache access --- and there's a reason for that. Atomic operations require a barrier between CPU cores, requiring the same inter-CPU access as an L2 cache access.

> The numbers can vary a bit when you have NUMA and multiple physical chips as well as cores to worry about --- but they are still good. The worst-case numbers for off-die contention can be in the 100s of nanosecods.