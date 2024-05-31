# Two Rules to Remember

We've mentioned these before, but they are *really important* to understanding what makes Rust tick - so here they are again.

* You can have an unlimited number of *immutable* references/accesses to a variable.
* **EXCLUSIVE OR** You can have ONE *mutable* reference/access to a variable.

You can never have both, and any code that fakes it is "unsound".

This is because - outside of atomics - updating a variable isn't a one-step process:

1. You read the value.
2. You update it.
3. You store the result.

If multiple threads can be performing these operations at the same time - data corruption/loss *will* occur.