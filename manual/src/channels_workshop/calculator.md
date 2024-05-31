# {optional} Task: A Decoupled Pocket Calculator

This one may seem like an odd task, but it's illustrating a common pattern.

1. Create an enumeration containing two calculator commands (add and subtract). Each will need to include an `f64`.
2. Create a `Command` structure that contains an operation and an `mpsc::Sender<f64>`.
3. In your `main` function:
    1. Create an MPSC channel that sends commands to the calculator.
    2. Create a second MPSC channel that receives responses.
    3. Spawn a thread that:
        1. Sets the current value to 0.
        2. Accepts commands and applies them to the current vaue.
        3. Sends a channel reply with the current value.
    4. Build a vector of some commands.
    5. Submit the commands.
    6. Print the responses.

![](../images/ScrollTime.png)

Here's my slightly over-cooked version:

```rust
use std::{sync::mpsc, thread};

enum Operation {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
}

struct Command {
    operation: Operation,
    tx: mpsc::Sender<f64>,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let (result_tx, result_rx) = mpsc::channel();

    // Spawn a thread to be the calculator actor
    thread::spawn(move || {
        let mut current_value = 0.0;

        while let Ok(command) = rx.recv() {
            match command.operation {
                Operation::Add(value) => current_value += value,
                Operation::Subtract(value) => current_value -= value,
                Operation::Multiply(value) => current_value *= value,
                Operation::Divide(value) => current_value /= value,
            }

            command.tx.send(current_value).unwrap();
        }
    });

    // Send some commands to the calculator actor
    let commands = vec![
        Command {
            operation: Operation::Add(10.0),
            tx: result_tx.clone(),
        },
        Command {
            operation: Operation::Subtract(5.0),
            tx: result_tx.clone(),
        },
        Command {
            operation: Operation::Multiply(2.0),
            tx: result_tx.clone(),
        },
        Command {
            operation: Operation::Divide(4.0),
            tx: result_tx.clone(),
        },
    ];

    let n_commands = commands.len();
    for command in commands {
        tx.send(command).unwrap();
    }

    // Receive the results
    for _ in 0..n_commands {
        println!("Result: {}", result_rx.recv().unwrap());
    }
}
```

# Why Do This?

> Couldn't we just use a struct with functions? Yes, you absolutely could.

This pattern is really common in systems that are expected to scale out, or are distributed between multiple teams. It's especially common in the async world (implemented as the actor model). Coincidentally, it's also how writing in Objective-C or Erlang feels.

* You've started with a modular monolith (a single codebase with lots of modules, often maintained by different groups who'd like to require frequent meetings). Effectively microservices with a single binary.
* You'd like to avoid the overhead of a TCP call to `localhost` every time you want to call another service. So you have two options:
    * Each service can expose a set of functions directly (note: there's nothing wrong with this!).
    * Services can expose channels that *look* like RPC calls via channels.
* When you want to talk to a "remote" service, you acquire it through an exposed function. This function:
    * Spawns a new instance of the actor.
    * Attaches the channels in each direction.
    * The actor and the client are now in direct communication.
* The interface has now been limited to:
    * A "connect" function that sets up one or more channels (you often don't need a reply for event-driven architectures).
    * A command pattern or notify pattern.
* From the caller's perspective, if there's no need for a synchronous reply then there's basicaly no performance hit to calling the "remote" API.

So far, so good. You've added a few microseconds of latency to receiving results, but nothing compared to the overhead of using a full TCP session (and a lot more than just calling directly).

Now lightning strikes and your service needs to scale up. Say one service in particular is particularly popular, so you'd like to move it to a different system.

* You replace the "connect" function with one that connects via Prost/Tonic (Prost compiles protobufs to Rust, Tonic provides an async protobuf client/server architecture).
* Great news! The Tonic system already uses channels to represent remote procedure calls - they just happen to be remote. So very little change (other than anything required to make it work with Protobuf) is required.
