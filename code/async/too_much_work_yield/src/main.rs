use std::time::Duration;

async fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
            tokio::task::yield_now().await;
        }
        true
    }
}

async fn find_prime() {
    let result = is_prime(999983).await;
    println!("Is prime: {}", result);
}

async fn spin() {
    for _ in 0 .. 5 {
        println!("Doing some work");
        tokio::time::sleep(Duration::from_secs_f32(0.01)).await;
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handle = tokio::spawn(spin());

    find_prime().await;

    let _ = handle.await;
}
