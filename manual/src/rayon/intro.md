# How Rayon Works

Rayon is a lot like *Intel Threaded Building Blocks*. It changes the unit of work from the "thread" to the "task"---a lot like an async setup, but without any async keywords.

Rayon is *not* great for long-lived threads, it's more like scoped threads (the standard library actually implemented them based on Rayon). So Rayon is optimized for number-crunching, more than control or messaging threads.

Rayon *can* live alongside other systems. You can have Rayon tasks along with threads and even async if you need to.

> Rayon is also quite bad at heavily I/O bound workloads. If a thread is "parked" waiting for IO, it can't run other tasks! Other threads can still steal its tasks - but you wind up with all threads waiting, you're slowing down to the speed of I/O operations.