use std::sync::mpsc;

enum Command {
    Execute {
        n: i32,
        func: fn (i32) -> i32,
        reply: oneshot::Sender<i32>,
    },
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Execute{n, func, reply} => {
                    let response = func(n*2);
                    reply.send(response).unwrap();
                }
                Command::Quit => break,
            }
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        let (reply_tx, reply_rx) = oneshot::channel();
        tx.send(
            Command::Execute {
                n: i,
                func: |i| i*3,
                reply: reply_tx,
            }
        ).unwrap();
            
        if let Ok(response) = reply_rx.recv() {
            println!("Response received: {response}");
        }
    }
    tx.send(Command::Quit).unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}