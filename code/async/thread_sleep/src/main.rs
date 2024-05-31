use std::time::Duration;

async fn counter(n: f32) {
    std::thread::sleep(Duration::from_secs_f32(1.0 - n));
    println!("{n}");
}

async fn correct_counter(n: f32) {
    tokio::time::sleep(Duration::from_secs_f32(1.0 - n)).await;
    println!("{n}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    for i in 0..10 {
        tokio::spawn(correct_counter(i as f32 / 10.0));
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
}