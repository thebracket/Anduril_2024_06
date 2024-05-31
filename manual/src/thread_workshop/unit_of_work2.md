# Unit of Work 2

The unit of work is still the operating system thread---the threads are just contained in a scope that makes "split this between CPUs" a lot easier. We haven't required a runtime, async or anything like that. It's just pure OS system calls.

Scoping helps because you can guarantee that the lifetime of captured items will outlive the thread scope.

> Scoped threads weren't originally part of the Rust standard library. They were implemented in a library called `Rayon`---and were so nice to have that the standard library adopted them.
