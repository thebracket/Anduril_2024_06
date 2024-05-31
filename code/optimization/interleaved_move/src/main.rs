use std::time::Instant;
use std::thread;
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

fn interleave_move(data: Vec<usize>, num_chunks: usize) -> Vec<Vec<usize>> {
    let mut chunks = Vec::with_capacity(num_chunks);
    for _ in 0 .. num_chunks {
        chunks.push(Vec::with_capacity(data.len() / num_chunks + 1));
    }

    for (i, item) in data.into_iter().enumerate() {
        chunks[i % num_chunks].push(item);
    }

    chunks
}

fn main() {
    let num_threads = thread::available_parallelism().unwrap();
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();
    let chunks= interleave_move(candidates, num_threads.into());

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    //let chunks = candidates.chunks(MAX_NUMBER / num_threads);
    let primes: Mutex<Vec<usize>> = Mutex::new(Vec::with_capacity(10000));
    thread::scope(|scope| {
        for chunk in chunks.iter() {
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