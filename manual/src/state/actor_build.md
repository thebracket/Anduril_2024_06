# Collaborative: build an actor

Let's work through building an actor framework together.

We'll start with a new project and add tokio (`cargo add tokio -F full`).

## Actor Skeleton

We can use a little bit of boilerplate to build a repeatable pattern for consistency.

```rust
// Normally a separate file/directory, maybe even a separate crate
mod stateful_actor {

    // Public commands to offer. Can also be wrapped in a "proxy" type with
    // public methods.
    pub enum StatefulActorCommands{}

    // You need to store a sender for the channel, or it will close. Storign
    // an idle channel is negligible overhead. `OnceLock` is perfect for this,
    // it initializes exactly once and cannot be replaced once set.
    static SENDER_STORAGE: OnceLock<Sender<StatefulActorCommands>> = OnceLock::new();

    // When a client wants to call the actor, `connect()` gives them a clone
    // of the TX channel.
    pub fn connect() -> Sender<StatefulActorCommands> {
        SENDER_STORAGE.get().cloned().unwrap()
    }

    // Start the actor on initialization. Any initial setup would go here.
    // Notice that we are *spawning* the `run` function - it launches in
    // a detached async future.
    pub async fn start() {
        let (sender_storage, receiver) = tokio::sync::mpsc::channel(100);
        SENDER_STORAGE.get_or_init(|| sender_storage);
        spawn(run(receiver));
    }

    // Run the main loop and call into `receive`. This could be
    // a member method. I sometimes keep it separate if there are
    // additional tasks to perform before getting into the state,
    // or if you also need to listen for a command to terminate
    // - and that requires more work.
    async fn run(mut receiver: Receiver<StatefulActorCommands>) {
        let state = StatefulActor::init();
        while let Some(command) = receiver.recv().await {
            state.receive(command).await;
        }
    }

    // The actor itself. "State" is just an atomic here, but could be
    // anything - or lots of fields.    
    struct StatefulActor {
        state: AtomicI32,
        // We're keeping a handle to our own Arc. Passing references
        // to `self` between async calls that might cross threads doesn't
        // work - so this is our "get out of lifetime jail" card.
        //
        // You only need this if you want to spawn handlers rather than
        // handling them sequentially.
        handle: OnceLock<Arc<Self>>,
    }
}
```

## Implementation

We'll build a simple adding machine for state:

```rust
mod stateful_actor {
    pub enum StatefulActorCommands {
        Add(i32),
        GetState(tokio::sync::oneshot::Sender<i32>)
    }

    impl StatefulActor {
        fn init() -> Arc<Self> {
            let me = Arc::new(
                StatefulActor { 
                    state: AtomicI32::new(0),
                    handle: OnceLock::new(),
                }
            );
            me.handle.get_or_init(|| me.clone());
            me
        }

        async fn receive(&self, command: StatefulActorCommands) {
            match command {
                StatefulActorCommands::Add(value) => {
                    self.state.fetch_add(value, std::sync::atomic::Ordering::Relaxed);
                }
                StatefulActorCommands::GetState(sender) => {
                    // In-process version
                    //let state = self.state.load(std::sync::atomic::Ordering::Relaxed);
                    //sender.send(state).unwrap();

                    // Out-of-process version
                    let handle = self.handle.get().unwrap().clone();
                    tokio::spawn(Self::get_state_out_of_process(handle, sender));
                }
            }
        }

        async fn get_state_out_of_process(handle: Arc<Self>, sender: tokio::sync::oneshot::Sender<i32>) {
            let state = handle.state.load(std::sync::atomic::Ordering::Relaxed);
            sender.send(state).unwrap();
        }
    }
}
```

## Test Harness

And finally some code to test it:

```rust
async fn actor_conversation() {
    // Connect to the actor.
    let sender = stateful_actor::connect();

    // Keep blasting commands to the actor.
    loop {
        sender.send(stateful_actor::StatefulActorCommands::Add(1)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // Start the actor.
    stateful_actor::start().await;

    // Start 5 conversations with the actor.
    for _ in 0..5 {
        tokio::spawn(actor_conversation());
    }

    // Sleep for 10 seconds.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Ask the actor for the count
    let sender = stateful_actor::connect();
    let (reply_sender, reply_receiver) = tokio::sync::oneshot::channel();
    sender.send(stateful_actor::StatefulActorCommands::GetState(reply_sender)).await.unwrap();
    let state = reply_receiver.await.unwrap();
    println!("Final state: {}", state);
    println!("{state} additions in ~2 seconds.");
    println!("{:.2} additions per second.", state as f64 / 2.0);
}
```

On my office workstation (core i7), I top out around 5,435,711 one-way commands per second, including the atomic increment. That's usually enough! (1,502,894 in debug mode). The channel is unlikely to be the bottleneck!

## Final Version

Here's a final version (see `/code/state/actor`):

```rust
mod stateful_actor {
    use std::sync::{atomic::AtomicI32, Arc, OnceLock};
    use tokio::{spawn, sync::mpsc::{Receiver, Sender}};

    pub enum StatefulActorCommands {
        Add(i32),
        GetState(tokio::sync::oneshot::Sender<i32>)
    }

    static SENDER_STORAGE: OnceLock<Sender<StatefulActorCommands>> = OnceLock::new();

    pub async fn start() {
        let (sender_storage, receiver) = tokio::sync::mpsc::channel(100);
        SENDER_STORAGE.get_or_init(|| sender_storage);
        spawn(run(receiver));
    }

    pub fn connect() -> Sender<StatefulActorCommands> {
        SENDER_STORAGE.get().cloned().unwrap()
    }

    struct StatefulActor {
        state: AtomicI32,
        handle: OnceLock<Arc<Self>>,
    }

    impl StatefulActor {
        fn init() -> Arc<Self> {
            let me = Arc::new(
                StatefulActor { 
                    state: AtomicI32::new(0),
                    handle: OnceLock::new(),
                }
            );
            me.handle.get_or_init(|| me.clone());
            me
        }

        async fn receive(&self, command: StatefulActorCommands) {
            match command {
                StatefulActorCommands::Add(value) => {
                    self.state.fetch_add(value, std::sync::atomic::Ordering::Relaxed);
                }
                StatefulActorCommands::GetState(sender) => {
                    // In-process version
                    //let state = self.state.load(std::sync::atomic::Ordering::Relaxed);
                    //sender.send(state).unwrap();

                    // Out-of-process version
                    let handle = self.handle.get().unwrap().clone();
                    tokio::spawn(Self::get_state_out_of_process(handle, sender));
                }
            }
        }

        async fn get_state_out_of_process(handle: Arc<Self>, sender: tokio::sync::oneshot::Sender<i32>) {
            let state = handle.state.load(std::sync::atomic::Ordering::Relaxed);
            sender.send(state).unwrap();
        }
    }

    async fn run(mut receiver: Receiver<StatefulActorCommands>) {
        let state = StatefulActor::init();
        while let Some(command) = receiver.recv().await {
            state.receive(command).await;
        }
    }
}

async fn actor_conversation() {
    // Connect to the actor.
    let sender = stateful_actor::connect();

    // Keep blasting commands to the actor.
    loop {
        sender.send(stateful_actor::StatefulActorCommands::Add(1)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // Start the actor.
    stateful_actor::start().await;

    // Start 5 conversations with the actor.
    for _ in 0..5 {
        tokio::spawn(actor_conversation());
    }

    // Sleep for 10 seconds.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Ask the actor for the count
    let sender = stateful_actor::connect();
    let (reply_sender, reply_receiver) = tokio::sync::oneshot::channel();
    sender.send(stateful_actor::StatefulActorCommands::GetState(reply_sender)).await.unwrap();
    let state = reply_receiver.await.unwrap();
    println!("Final state: {}", state);
    println!("{state} additions in ~2 seconds.");
    println!("{:.2} additions per second.", state as f64 / 2.0);
}
```