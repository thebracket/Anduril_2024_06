# Rayon and Iterators

You saw an example of using Rayon's parallel iterators. In most cases, you can drop `par_iter` in where you would use `iter`. 

> Remember that even with a premade thread pool, moving work to threads isn't free---it is still often faster to use a single threaded iterator, especially on small amounts of data!

Regular iterator example:

```rust
use rayon::prelude::*;

fn main() {
    (0..100).into_par_iter().for_each(|x| println!("{x}"));
}
```

When you do this, you are dividing each entry into a "task", one task per element. The thread pool can chew through these fast, but you can often achieve even better performance by using some of Rayon's "indexed iterator" chunking functions. After `.par_iter()` (or `.into_par_iter()`) you can add `.uniform_locks(block size)` to group tasks to cover a number of items, or `by_exponential_blocks()` (which is often faster for search functions).