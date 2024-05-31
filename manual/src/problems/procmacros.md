# Procedural Macros

> I admit it. I saw "proc macros" as a question and thought "wow, I'll be here all month". So this is a *condensed* approach that is going to point you at resources and only skim the surface. Procedural macros are an *enormous* topic.

There's a really great tutorial here: [https://www.freecodecamp.org/news/procedural-macros-in-rust/](https://www.freecodecamp.org/news/procedural-macros-in-rust/)

## When to use Macros

It's *really* tempting to take the macros and turn Rust into something not-very-Rust-like at all. This is the LISP way (step 1 of writing a large LISP program is to build a language that suits your problem)---and it's not a great idea. You'll have a terrible time onboarding newcomers to your "here's the language we built, and here's the program to work on" approach. Use macros where you *need* them to extend the language, or make something easier.

Also, macros compile pretty slowly. You can really hurt your compilation times with a lot of macros.

So, use macros sparingly - and enjoy the power they provide.

## Types of Macro

The macros you see the most are things like `println!` --- **Declarative** macros. These give you great metaprogramming and relatively easy syntax extension. You define these with `macro_rules!` statements. We're not going to dive into these.

Procedural macros receive the AST (Abstract Syntax Tree) tokens and modify it at compile time. This lets you modify pretty much anything! Proc Macros are either:

* `derive` macros - when you use `#[derive(Whatever)]` - you are invoking a derive procedural macro.
* `attribute` macros. When you see a function with `#[must-use]` - that's an attribute macro. The tracing crate uses `#[instrumentation]` to add in telemetry.
* `functional` macros are macros disguised as functions. You don't "tag" these - but you call them like a function.

## Crates

Procedural macros only work for projects marked as such in `Cargo.toml`. Once you've added this mark, it's problematic to *also* have functionality. So you almost always end up with a crate for your procedural macros, and a crate for your library. You sometimes end up needing three (or more) crates to avoid circular dependencies.

That's why when you include `serde` with the `derive` flag, you see a `serde` crate *and* a `serde_derive` crate in your dependency tree: the procedural macros *must* be isolated.

## Helpful Libraries

Reading and writing an AST token stream isn't a lot of fun (well, maybe someone likes it). Pretty much everybody who isn't David Tolnay (author of many of these crates and Serde) use some dependencies:

* `syn` parses token streams into Rust syntax.
* `quote` parses Rust syntax into token streams.
* `proc-macro2` exposes all of the standard libary proc-macro features, including some that the standard library doesn't directly expose. It's planned to fix that, but for now---everyone uses `proc-macro2`.
* `darling` is a helper that makes handling proc macro arguments a LOT less cumbersome.

## So... let's make a Derive Macro

> The code for this is in `code/procmacros/`

Pick a parent directory, and make two new cargo projects:

```bash
cargo new deriver
cargo new deriver-macros --lib
```

Now open `deriver-macros/Cargo.toml` and tell it to be a procedural macro crate:

```toml
[package]
name = "deriver-macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
```

And in `deriver/Cargo.toml` make it depend upon the library:

```toml
[package]
name = "deriver"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
deriver-macros = { path = "../deriver-macros" }
```

Then add some dependencies to `deriver-macros`:

```bash
cd deriver-macros
cargo add syn quote proc-macro2 darling
```

### A Macro Stub

Replace `deriver-macros/src/lib.rs` with the following:

```rust
// deriver-macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    todo!("We haven't written this yet");
}
```

You *have* to use the `extern crate` system to link the standard library's `proc_macro` system. Then you need to use a `TokenStream` - which represents the actual AST tokens. Your function will return a modified token stream.

The `#[proc_macro_derive]` macro marks that this function is a derive macro and names it. We've left the implementation as a `todo!`.

Now over in `deriver/src/main.rs`, let's show how we will use the macro:

```rust
use deriver_macros::HelloMacro;

#[derive(HelloMacro)]
struct MyData {
    name: String,
}

fn main() {
    println!("Hello, world!");
}
```

The compiler will show a "not implemented yet" - but we've linked the macro to the derive call.

### Start by Scanning the Structure

I've added comments inline for each section of what we're doing here:

```rust
// deriver-macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Data;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    // Read the input and use `syn` to parse it into something we can work with.
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    // Identify the struct to which the macro is being applied.
    let struct_id = &input.ident;

    // We can read "input.data" to get the fields of the struct.
    match input.data {
        // A struct type with named fields. The `fields` entry
        // contains a list of the fields.
        Data::Struct(syn::DataStruct { fields, ..}) => {
            todo!("We haven't written this yet")
        }
        _ => unimplemented!(),
    }

    todo!("We haven't written this yet");
}
```

So basically we obtain the structure's identity. Then we examine the Syn parsed data type, and if its actually a structure -- obtain information about it.

### Emit Some Rust

Now that we can introspect the structure, let's actually write some Rust:

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_id = &input.ident;

    match input.data {
        Data::Struct(syn::DataStruct { fields, ..}) => {
            // The `quote!` macro allows you to write Rust instead of
            // trying to write AST entries yourself. It's a lot easier!
            //
            // We'll build an implementation block first.
            let mut implementation = quote!{
                println!("I am a struct of type [{}]", stringify!(#struct_id));
            };

            // Now we'll iterate over the fields and add a print statement
            // for each one.
            for field in fields {
                let field_name = field.ident.unwrap();
                implementation.extend(quote!{
                    println!("Field: [{}] = [{}]", 
                        stringify!(#field_name),
                        self.#field_name,
                    );
                });
            }

            // Now we'll embed the implementation block into the final
            // output.

            quote! {
                impl #struct_id {
                    fn hello_macro(&self) {
                        #implementation
                    }
                }
            }.into() // The .into() is necessary to convert the output back into a token stream.
        }
        _ => unimplemented!(),
    }
}
```

We're using the `quote!` macro to let us write Rust and have it parsed into a `TokenStream` (writing AST tokens by hand isn't exactly fun, and it's also fraught with error --- the crate uses Rust compiler mechanisms, so your macros keep up with updates).

We start by building an `implementation` variable. It contains the Rust required to print "I am a struct of type [type name]".

Then we extend the implementation by iterating through the fields, and adding a print statement for each one. (This macro won't work for any type that doesn't implement `Display`!).

Finally, we emit a `quote` describing the implementation block we want - and embed `#implementation` into the middle.

Now we can update `main.rs` and try it:

```rust
use deriver_macros::HelloMacro;

#[derive(HelloMacro)]
struct MyData {
    name: String,
}

fn main() {
    let person = MyData {
        name: "Alice".to_string(),
    };
    person.hello_macro();
}
```

The output is:

```
I am a struct of type [MyData]
Field: [name] = [Alice]
```

### What Have We Achieved?

* We've added compile-time introspection.
* We've written a fair amount of boilerplate that is unavoidable until the reflection proposals make some progress (they are doing as well as the C++ reflection papers!)
* From the library consumer's point of view, we've saved a lot of typing by allowing a simple `Derive` macro to customize code just for our structure.

This is *really* powerful. It's also really easy to abuse. It's quite similar to what you can get up to with templates in C++ if you feel like it. At least you can't `#define true (__LINE__ % 10 != 0)`...

