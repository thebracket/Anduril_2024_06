use std::time::Instant;
use std::thread::{self, available_parallelism};
use std::sync::Mutex;

const MAX_NUMBER: usize = 100_000;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let num_threads = available_parallelism().unwrap();
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let chunks = candidates.chunks(MAX_NUMBER / num_threads);
    let primes: Mutex<Vec<usize>> = Mutex::new(Vec::with_capacity(10000));
    thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(|| {
                let local_results: Vec<usize> = chunk
                    .iter()
                    .filter(|n| is_prime(**n))
                    .copied()
                    .collect();

                let mut lock = primes.lock().unwrap();
                lock.extend(local_results);
            });
        }
    });
    let elapsed = start.elapsed();

    // Results
    let lock = primes.lock().unwrap();
    println!("Found {} primes", lock.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}