# Spot the Difference 3


One contains an `&` and the other doesn't.

When you run the version with the `&`, you get:

```
hello
```

When you run the version without the `&`, you get:

```
A copy happened!
```

*(Or "hello" with no indication that you just wasted time on a copy, in the sneaky version)*

That's why Rust makes it explicit that you *have* to decorate borrowing on both the caller side and the callee side. The second one didn't even print "hello", because `std::string` wasn't copied in a copy constructor.

So yes---Rust *is* making you do more typing. But it's also ensuring that you don't accidentally lose some data because you forgot all 5 constructors, or a simple misunderstanding changed the call semantics of a function.