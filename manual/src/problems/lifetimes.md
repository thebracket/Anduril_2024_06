# Lifetimes

The full question was:

> In-depth covering of lifetimes (what does the compiler do with your declared lifetimes)

Let's start with the easy part: Lifetimes (along with `mut`, `PhantomData` and a few others) don't actually do *anything at all* in the compiled binary---they are compile-time constructs only, used to determine if Rust thinks the program is sufficiently safe to allow it to compile.

## A Little History

When Rust was first being demoed (it was *very* different!), every reference made you list a lifetime. You still can, if you feel masochistic:

```rust
fn do_something<'a>(n: &'a i32) {
    println!("{n}");
}

fn main() {
    let i = 32;
    do_something(&i);
}
```

Rust introduced *lifetime elision* to reduce the pain. When a lifetime is obvious (to the compiler), it quietly adds it for you anyway. So when you type `fn do_something(n: &i32)`---Rust really is compiling `fn do_something<'anonymous>(n: &'anonymous i32)`.

## So What Does it Do?

Let's start with the very simple case:

```rust
fn do_something(n: &String) {
    println!("{n}");
}

fn main() {
    let i = "Hello".to_string(); // The "lifetime" for `i` starts here

    {
        let ref_i = &i;         // Borrow starts
        do_something(ref_i);
        // Borrow leaves scope and ends
    }

    {
        let ref_j = &i;         // Borrow starts
        do_something(ref_j);
        // Borrow leaves scope and ends
    }

    // `i` falls out of scope here, it's lifetime ends
}
```

(Credit: [Rust by Example](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)).

So *every* variable has an associated lifetime, and every borrow has a lifetime. Borrow lifetimes are parented on the lifetime from which they are borrowing. This isn't in the final output code, but the compiler is doing this for everything---to prevent you from having any references that outlive their parent.

So to expand that to be equivalent of the C++ issue:

```rust
fn do_something(n: &String) {
    println!("{n}");
}

fn main() {
    let i = "Hello".to_string();    // The "lifetime" for `i` starts here
    let ref_i = &i;         // Borrow starts here
    do_something(ref_i);    // Borrow is fine
    std::mem::drop(i);              // The parent "i" lifetime just ended.
                                    // The compiler treats it as a "move"
                                    // (hence the error message)
    do_something(ref_i);    // So this won't work, because `i` is gone
}
```

This doesn't compile.

How about a trickier one?

```rust
fn main() {
    let x = "Hello".to_string();      // "Lifetime of x starts"
    let z;          
    let y = &x;     // "Lifetime of y starts, and x is extended"
    z = y;          // Z starts, y and x are extended

    std::mem::drop(x);  // x dies, making the other references invalid
    println!("{z}");
}
```

Rust is doing good work here: it's making absolutely sure that you aren't referring to something that isn't around anymore - giving at best undefined behavior.

## When Lifetimes Get Tricky

Take this innoccuous looking code that trips up everyone who starts using Rust:

```rust
fn main() {
    let x = 42;
    let handle = std::thread::spawn(|| {
        // x is "captured" - which is a borrow from the parent scope
        println!("{x}");
    });
    handle.join();
}
```

This doesn't compile, with the cryptic message `closure may outlive the current function, but it borrows *x*, which is owned by the current function`.

> As you saw earlier, scoped threads are the way to avoid this.

The problem is that Rust can't be sure which thread will die first---so it has no way of knowing if the reference to `x` is still valid at the end of the function. Making `x` static, or a constant will work fine. Moving it works fine, too - because it's no longer a reference, it's an owned value.

## When to Annotate Lifetimes

> Most of the time, Rust figures out lifetime annotation for you. Several prominent Rustaceans have said "if you start needing to annotate lifetimes regularly, it's usually a sign that you need to rethink some code" (not always)

Rust keeps getting better at eliding lifetimes. Not too many versions ago, this would have required an annotation:

```rust
fn bigger(a: &i32, b: &i32) {
    if a > b {
        println!("Bigger");
    }
}

fn main() {
    let a = 1;
    let b = 2;
    bigger(&a, &b);
}
```

Currently, you really only need to annotate lifetimes if the compiler needs a hint - or if you are returning a reference. If you are returning a reference, it *must* be one of the listed lifetimes (fortunately, you can specify that both inputs are on the same lifetime for the function's purposes). This won't compile without specifying a lifetime:

```rust
fn bigger<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 1;
    let b = 2;
    println!("{}", bigger(&a, &b));
}
```

Structures that store a reference *must* annotate the lifetime:

```rust
#[derive(Debug)]
struct Borrowed<'a>(&'a String);

fn main() {
    let s = "Hi".to_string();
    let borrowed = Borrowed(&s);
    println!("{borrowed:?}");
    // Uncomment to prove that lifetimes still work
    //std::mem::drop(s);
    //println!("{borrowed:?}");
}
```

## An Example of Lifetime Failure

```rust
#[derive(Debug)]
struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a mut Cat
}

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];
    
    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder{ cat })
    }
    
    feeders.iter_mut().for_each(|f| f.feed());
    println!("{cats:?}");
}
```

This is a common enough pattern: cat feeders hold a reference to a child object (that they feed). It works fine, because everything is in scope.

Let's add a scope just to break it:

```rust
#[derive(Debug)]
struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a mut Cat
}

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut feeders = Vec::new(); // Moved to the top so it's in scope
    {
        let mut cats = vec![                // 'CATS begins
            Cat("Frodo".to_string()),
            Cat("Bilbo".to_string()),
            Cat("Pippin".to_string()),
        ];
        
        for cat in cats.iter_mut() {
            feeders.push(CatFeeder{ cat })
        }
    }                                       // 'CATS ends
    
    // Because the lifetime of cats is ended, the references - which refer to 'CAT
    // are invalid. So it won't compile.
    feeders.iter_mut().for_each(|f| f.feed());
}
```

`cats` doesn't live long enough. We've passed references to each cat to the feeder, and then taken `cats` away.

## Lifetimes in Async

Async actually works pretty well with lifetimes. Tokio---and anything that allows `async` to jump threads with `spawn`---can make it a little painful.

So this works fine:

```rust
struct Test;

impl Test {
    async fn call_me(&self) {
        self.maybe().await;
    }

    async fn maybe(&self) {

    }
}

#[tokio::main]
async fn main() {
    let t = Test;
    t.call_me().await;
}
```

And this doesn't even compile:

```rust
struct Test;

impl Test {
    async fn call_me(&self) {
        tokio::spawn(self.maybe());
    }

    async fn maybe(&self) {

    }
}

#[tokio::main]
async fn main() {
    let t = Test;
    t.call_me().await;
}
```

You get the rather cryptic error `borrowed data escapes outside of method` (with nice illustrations to make it less cryptic). You're borrowing `self`, and then calling `&self.maybe()` in `spawn` - which can bump your task to a different thread. In the Actors demo, we used an `Arc` to get around this. In the current edition and version of Tokio---you may need to do that a lot!

So let's try that as a free function:

```rust
struct Test;

async fn call_me(t: &Test) {
    tokio::spawn(maybe(t));
}

async fn maybe(_t: &Test) {

}

#[tokio::main]
async fn main() {
    let t = Test;
    t.call_me().await;
}
```

It *still* doesn't work. Basically, you can't pass references through `spawn`.

> There is work to improve this a lot in the next edition of Rust. For now, lifetimes and async are a battle. Passing references across "spawn" calls is extremely problematic.

### Pinning

The solution is to use the `Pin` system. You *pin* a variable in memory, guaranteeing that it won't move---and *then* you can pass references. This is really unwiedly! Here's an example of a "stream adapter"---which *must* use references to the streams it is adapting:

```rust
use pin_project_lite::pin_project;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio_stream::StreamExt;

pin_project! {
    struct ToUpper {
        #[pin]
        stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>,
    }
}

impl ToUpper {
    fn new(stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>) -> Self {
        Self { stream }
    }
}

impl tokio_stream::Stream for ToUpper {
    type Item = std::io::Result<String>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx).map(|opt| {
            opt.map(|res| {
                res.map(|line| {
                    line.to_uppercase() + "\n"
                })
            })
        })
    }
}

#[tokio::main]
async fn main() {
    let file = tokio::fs::File::open("Cargo.toml").await.unwrap();
    // convert the `AsyncRead` into a buffered reader, then a line stream, then your adapter
    let stream = BufReader::new(file).lines();
    let stream = tokio_stream::wrappers::LinesStream::new(stream);
    let mut stream = ToUpper::new(stream);
    while let Some(line) = stream.next().await {
        print!("{}", line.unwrap());
    }
}
```

We're actually using the `pin_project` crate to make the syntax *easier*.

So my advice is, if you are making use of async spawning---avoid references. And be very careful of references in async in general.