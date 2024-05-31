# Concurrent State Management

With Rust's emphasis on concurrency, state management can become a little more complicated. There are a LOT of different patterns, suited to different problem spaces.

We'll work through a few of them.

## The Trouble with Storing References

Some C++ programs tend to pass state around as a series of contexts, storing direct references. I've seen this especially in game development. You'll have classes everywhere with a stashed `&GameState`, `&Render` and similar. Rust can make this tricky, because it tracks the lifetime of each reference---so you have to be able to prove that the classes are going to out-live everything to which they hold a reference.

In almost every occasion, in Rust you'd instead store an `Arc<GameState>` or `Arc<Render>` and handle changes through synchronized interior mutability. Arc is *very* cheap to clone.

You *can* use the `&'static GameState` to pinky-swear that `GameState` is static---but you are opting out of some of the protections by doing so. Unless a few nanoseconds is *critical* to your program, it's worth retaining the safety.