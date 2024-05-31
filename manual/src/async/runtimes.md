# Async Runtimes

Just like C++, Rust opted to not implement an "async runtime" in the standard library. This gives you flexibility: different runtimes work better for different tasks. Some runtimes:

Runtime|Comment
--|--
Futures|The reference standard. It's only half a runtime; it lacks an executor (translating IO calls to async polling)
Tokio|The de-facto standard for most Enterprise setups. Provides a huge library.
Glommio|Linux-only, implements IO_URING for extreme performance.
Smol|A really tiny async runtime
Oxide|A tiny async runtime that works well in embedded systems
Embassy|An entire async operating system for really small embedded systems!

We're going to use Tokio, because it's the most common runtime.