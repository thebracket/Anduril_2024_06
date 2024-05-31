# Let's Make a Workspace

We're going to build a lot of smaller projects. Let's use a workspace.

> Workspaces provide a way to group your code together, and share dependencies so they don't contionually recompile - and leave you with vast swathes of wasted disk space on duplicate items.

Let's create a workspace.

1. `cd` to your parent directory.
2. Create a new Rust project with `cargo new my_workspace`.
3. `cd` into `my_workspace`.
4. Edit `src/main.rs` to change "Hello, World!" to something like "You probably intended to run a workspace member". This is optional, but helps avoid confusion.
5. While in `my_workspace`, create a new project. `cargo new hello`.
6. Edit `my_workspace/Cargo.toml`:

```toml
[workspace]
members = [ "hello" ]
```

> Note: recent Rust will add this for you when you do `cargo new` inside an existing workspace. It tends to make a mess of your `Cargo.toml` file---so it's worth taking the time to organize it.

Now change directory to `my_workspace/hello` and run the program with `cargo run`.

Take a look at `my_workspace` and you will see that a `target` directory has appeared. Within a workspace, all compiler artifacts are shared. For large projects, this can save a huge amount of disk space. It can also save on re-downloading dependencies, and will only recompile portions of the workspace that have changed.

> While working on *Hands-on Rust*, I initially had 55 projects in separate crates without a workspace. I noticed that my book's `code` folder was using nearly 6 gigabytes of disk space, which was crazy. So I added a workspace, and that shrunk to a few hundred megabytes. Every single project was downloading all of the dependencies and building them separately.

Workspaces are safe to upload to `github` or your preferred Git repo. You can even access dependencies within a workspace remotely. Build artefacts are shared (and you can define cross-workspace dependencies, too!)