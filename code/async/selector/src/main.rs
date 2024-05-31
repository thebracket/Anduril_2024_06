async fn ticker(tx: tokio::sync::mpsc::Sender<i32>) {
    loop {
        tx.send(1).await;
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}

async fn quitter(tx: tokio::sync::mpsc::Sender<i32>) {
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.5)).await;
        tx.send(0).await;
}

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<i32>(10);

    tokio::spawn(ticker(tx1));
    tokio::spawn(quitter(tx2));

    loop {
        tokio::select! {
            Some(val) = rx1.recv() => {
                println!("From 1: {val}");
            }
            Some(..) = rx2.recv() => {
                break;
            }
        }
    }
    println!("Quitting");
}