# Async Rust

Async sometimes get a bad name---it was added to Rust after the 1.0 release, and isn't as complete feeling as regular Rust. It offers *very* high performance for I/O (particularly network) bound workloads---and isn't always great for CPU bound workloads.

Technique|Unit of Work|Best For
--|--|--
`std::thread::spawn`|Thread|Long-running threads and thread pools.
`std::thread::scope`|Thread|Combined threads for a workload.
`rayon`|Task|Task-based workload division.
async|Future|I/O and not stalling while waiting for other systems.

> Async is very similar conceptually to coroutines in C++

## Function Coloring

A `sync` (regular) function can only call regular `sync` functions.

An `async` function can call `async` or `sync` functions.

This can lead to a "coloring" problem, where the async portions of your program spread - or where you just use async to access an async-only library. A bit of care and thought is required!

## Mixing the Two

You *can* have sync and async portions of your program. It's very common. The two most common patterns:

* `Async` program that spawns threads for long-running tasks or blocking items.
* A `Sync` program that has a small async portion to handle communication with other systems.

Communication between the two is frequently performed with channels.
