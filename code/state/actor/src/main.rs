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
