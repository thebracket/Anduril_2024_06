# Scopes


You already know this one! Standard library scopes were inspired by Rayon scopes. The only difference remains that you are adding tasks and not actual threads.

```rust
use rayon::prelude::*;

fn main() {
    let i = 5;
    rayon::scope(|scope| {
        scope.spawn(|_scope| println!("{i}"));
    });
}
```

This is the same example from earlier---but with Rayon instead of threads. There are two differences:

* `thread::scope` is `rayon::scope`.
* Each `scope.spawn` passes the `scope` as a variable downstream - so all of your sub-tasks remain in the same scope.
