use rayon::prelude::*;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0 .. 1_000_000).map(|_| rng.gen_range(0 .. 1_000_000)).collect();
    let mut numbers2 = numbers.clone();

    let now = std::time::Instant::now();
    numbers.sort();
    println!("Single-Thread: {:.4} ms", now.elapsed().as_millis());

    let now = std::time::Instant::now();
    numbers2.par_sort();
    println!("Multi-Thread: {:.4} ms", now.elapsed().as_millis());
}